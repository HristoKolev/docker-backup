use std::path::Path;

use serde::{Serialize, Deserialize};

use crate::global::prelude::*;
use crate::archive_helper::{ArchiveMetadata, read_metadata};
use crate::archive_type::{ArchiveType, get_remote_config};

pub fn upload_archive(archive_metadata: &ArchiveMetadata, remote_config: &RemoteConfig) -> Result {

    let local_file_path = archive_metadata.full_path.get_as_string()?;

    bash_exec!(
        "rclone copy -P {} {}:{}/",
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

#[derive(Debug)]
pub struct RemoteArchiveMetadata {
    pub archive_metadata: ArchiveMetadata,
    pub remote_config: RemoteConfig,
    pub remote_file: RemoteFile,
}

pub fn list_remote_archives(archive_type: Option<&ArchiveType>) ->  Result<Vec<RemoteArchiveMetadata>> {

    let archive_types = match archive_type {
        Some(x) => vec![x.clone()],
        None => ArchiveType::all(),
    };

    let mut metadata = Vec::new();

    for archive_type in archive_types {

        let remote_configs = get_remote_config(&archive_type);

        for remote_config  in remote_configs {

            let response = bash_exec!(
                "rclone lsjson {}:{}/",
                remote_config.remote_name,
                remote_config.remote_path
            );

            let remote_files: Vec<RemoteFile> = serde_json::from_str(&response.stdout)?;

            for remote_file in remote_files {

                let full_path = Path::new(&remote_config.remote_path)
                    .combine_with(&remote_file.name);

                match read_metadata(&full_path)? {
                    None => (),
                    Some(archive_metadata) => {

                        if archive_metadata.archive_type == archive_type {

                            metadata.push(RemoteArchiveMetadata {
                                remote_config: remote_config.clone(),
                                remote_file,
                                archive_metadata
                            })
                        }
                    }
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