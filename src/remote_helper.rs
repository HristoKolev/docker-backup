use std::path::Path;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use time::Duration;
use chrono::Datelike;

use crate::global::prelude::*;
use crate::archive_helper::{ArchiveMetadata, read_metadata};
use crate::archive_type::{ArchiveType, get_remote_config};

pub fn upload_archive(archive_metadata: &ArchiveMetadata, remote_config: &RemoteConfig) -> Result {

    let local_file_path = archive_metadata.full_path.get_as_string()?;

    bash_exec!(
        "rclone copy {} {}:{}/",
        local_file_path,
        remote_config.remote_name,
        remote_config.remote_path
    );

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteFile {

    #[serde(rename = "Path")]
    pub path: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Size")]
    pub size: u64,

    #[serde(rename = "MimeType")]
    pub mime_type: String,

    #[serde(rename = "ModTime")]
    pub modification_time: String,

    #[serde(rename = "IsDir")]
    pub is_directory: bool,

    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct RemoteArchiveMetadata {
    pub archive_metadata: ArchiveMetadata,
    pub remote_config: RemoteConfig,
    pub remote_file: RemoteFile,
}

pub fn list_remote_archives(archive_type: Option<&ArchiveType>) ->  Result<Vec<RemoteArchiveMetadata>> {

    let archive_types = archive_type
        .map(|x| vec![x.clone()])
        .unwrap_or_else(|| ArchiveType::all());

    let mut metadata = Vec::new();

    for archive_type in archive_types {

        let remote_configs = get_remote_config(&archive_type);

        for remote_config  in remote_configs {

            let response = bash_exec_no_log!(
                "rclone lsjson {}:{}/",
                remote_config.remote_name,
                remote_config.remote_path
            );

            let remote_files: Vec<RemoteFile> = serde_json::from_str(&response.stdout)?;

            for remote_file in remote_files {

                let full_path = Path::new(&remote_config.remote_path)
                    .join(&remote_file.name);

                match read_metadata(&full_path)? {
                    Some(archive_metadata) => {

                        if archive_metadata.archive_type == archive_type {

                            metadata.push(RemoteArchiveMetadata {
                                remote_config: remote_config.clone(),
                                remote_file,
                                archive_metadata
                            })
                        }
                    },
                    None => (),
                };
            }
        }
    }

    Ok(metadata)
}

pub fn delete_remote_archive(remote_archive: &RemoteArchiveMetadata) -> Result {

    bash_exec!(
        "rclone deletefile {}:{}",
        remote_archive.remote_config.remote_name,
        remote_archive.archive_metadata.full_path.get_as_string()?
    );

    Ok(())
}

pub fn clear_remote_cache(archive_type: &ArchiveType) -> Result {

    log!("Clearing remote cache...");

    let all_archives = list_remote_archives(Some(archive_type))?;

    let map: HashMap<String, Vec<_>> = (&all_archives).into_iter()
        .filter(|x| x.archive_metadata.archive_date < (*app_start_time() - Duration::days(x.remote_config.cache_expiry_days)))
        .filter(|x| !are_permanent_archives_enabled(x) || !is_permanent(x))
        .order_by(|x| x.archive_metadata.archive_date)
        .group_by(|x| x.remote_config.remote_name.clone())
        .collect();

    for (remote_name, archives) in map {

        let remote_config = get_remote_config(archive_type).into_iter()
            .filter_first(|x|x.remote_name == remote_name)
            .or_error(&format!("No remote found with this name. Name: {}", remote_name))?;

        let take_count = if ((all_archives.len() as i32) - remote_config.min_archive_count) < 0 {0} else {((all_archives.len() as i32) - remote_config.min_archive_count)};

        let for_delete = archives
            .into_iter()
            .take(take_count as usize)
            .collect_vec();

        for remote_archive_metadata in for_delete {

            log!(
                "Deleting remote file: `{}:{}` ...",
                remote_archive_metadata.remote_config.remote_name,
                remote_archive_metadata.archive_metadata.full_path.get_as_string()?
            );

            delete_remote_archive(&remote_archive_metadata)?;
        }
    }

    Ok(())
}

fn is_permanent(metadata: &RemoteArchiveMetadata) -> bool {

    metadata.archive_metadata.archive_date.day() == 1
        || metadata.archive_metadata.archive_date.day() == 15
}

fn are_permanent_archives_enabled(metadata: &RemoteArchiveMetadata) -> bool {

    !metadata.remote_config.disable_permanent_archives.unwrap_or(false)
}