use std::collections::HashMap;

use time::Duration;
use clap::Arg;

use crate::global::prelude::*;
use crate::archive_type::*;
use crate::archive_helper::{list_local_archives, ArchiveMetadata};
use crate::remote_helper::{list_remote_archives, upload_archive, RemoteArchiveMetadata};

struct UploadCommandOptions {
    archive_type: Option<ArchiveType>,
}

fn upload_command_options() -> Result<UploadCommandOptions> {

    const ARCHIVE_TYPE_VALUE: &str = "archive-type";

    let matches =  cli().command_config(|x| {

        x.arg(Arg::with_name(ARCHIVE_TYPE_VALUE)
            .short("t")
            .long(ARCHIVE_TYPE_VALUE)
            .value_name(ARCHIVE_TYPE_VALUE)
            .help("The type of archive you want upload.")
            .required(false)
            .takes_value(true)
        )
    });

    let archive_type = matches.value_of(ARCHIVE_TYPE_VALUE)
        .map_result(|x| parse_archive_type(x))?;

    Ok(UploadCommandOptions {
        archive_type
    })
}

pub fn upload_command() -> Result {

    let options = upload_command_options()?;

    let local_archives = list_local_archives(options.archive_type.as_ref())?;

    let all_remote_archives = list_remote_archives(options.archive_type.as_ref())?;

    let remote_map: HashMap<String, Vec<_>> = all_remote_archives.into_iter()
        .group_by(|x| x.remote_config.remote_name.clone())
        .collect();

    let mut results = Vec::new();

    for (remote_name, remote_archives) in remote_map {

        let type_map: HashMap<ArchiveType, Vec<_>> =  remote_archives.into_iter()
            .group_by(|x| x.archive_metadata.archive_type.clone()).collect();

        for (archive_type, remote_archives) in type_map {

            let remote_config = get_remote_config(&archive_type)
                .into_iter().first(|x|x.remote_name == remote_name)
                .ok_or_else(|| CustomError::from_message(&format!("No remote found with this name. Name: {}", remote_name)))?;

            results.push(process_remote(&local_archives, &remote_archives, &remote_config));
        }
    }

    for result in results {
        result?;
    }

    Ok(())
}

fn process_remote(local_archives: &Vec<ArchiveMetadata>,
    remote_archives: &Vec<RemoteArchiveMetadata>,
    remote_config: &RemoteConfig) -> Result {

    for local_archive in local_archives {

        let is_expired = local_archive.archive_date < (*app_start_time() - Duration::days(remote_config.cache_expiry_days));

        let is_already_uploaded = remote_archives.into_iter()
            .any_result(|x| Ok(x.archive_metadata.full_path.file_name_as_string()? == local_archive.full_path.file_name_as_string()?))?;

        if !is_expired && !is_already_uploaded {
            upload_archive(local_archive, &remote_config)?;
        }
    }

    Ok(())
}