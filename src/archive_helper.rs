use std::path::{Path, PathBuf};

use uuid::Uuid;
use chrono::Utc;
use chrono::offset::TimeZone;
use clap::Arg;

use crate::global::{do_try, app_config};
use crate::global::prelude::*;
use sentry::internals::DateTime;

pub fn create_archive<F>(prefix: &str, func: F) -> Result
    where F: FnOnce(&str) -> Result {

    let app_config = app_config();

    let work_path = Path::new(&app_config.archive_config.temp_path)
        .combine_with(&Uuid::new_v4().to_string());

    do_try::run(|| {

        bash_exec!("mkdir -p {0} && chmod 777 {0}", &work_path.get_as_string()?);

        let uncompressed = work_path
            .combine_with("uncompressed-archive")
            .get_as_string()?;

        bash_exec!("mkdir -p {0} && chmod 777 {0}", uncompressed);

        func(&*uncompressed)?;

        let compressed = work_path
            .combine_with("compressed-archive.tar.gz")
            .get_as_string()?;

        bash_exec!(
            "cd {0} && tar -cf {1} --use-compress-program=pigz *",
            uncompressed,
            compressed
        );

        bash_exec!("rm {0} -rf", uncompressed);

        let final_archive = work_path
            .combine_with("final.enc")
            .get_as_string()?;

        bash_exec!(
            "openssl enc -aes-256-cbc -e -p -pass pass:{0} -in {1} -out {2}",
            &app_config.archive_config.archive_password,
            &compressed,
            &final_archive
        );

        bash_exec!("rm {0} -f", compressed);

        let now = app_start_time();

        let daily_folder = Path::new(&app_config.archive_config.cache_path)
            .combine_with(&now.format("day_%Y_%m_%d").to_string())
            .create_directory()?;

        let archive_file_name = format!(
            "{}.{}.{}.backup",
            prefix,
            now.format("%Y-%m-%d").to_string(),
            now.timestamp().to_string()
        );

        let archive_file_path = daily_folder
            .combine_with(&archive_file_name)
            .get_as_string()?;

        bash_exec!("mv {} {}", &final_archive, &archive_file_path);

        Ok(())
    }).finally(|| {

        bash_exec!( "rm {} -rf", work_path.get_as_string()?);

        Ok(())
    })?;

    Ok(())
}

pub fn list_archives(prefix: &str) -> Result<Vec<ArchiveMetadata>> {

    let app_config = app_config();

    let mut archives = Vec::new();

    let daily_folders = ::std::fs::read_dir(&app_config.archive_config.cache_path)?;

    for daily_folder_result in daily_folders {

        let daily_folder = daily_folder_result?.path();

        for archive_file_result in ::std::fs::read_dir(daily_folder)? {

            let archive_path = archive_file_result?.path();

            match read_metadata(&archive_path)? {
                Some(x) => archives.push(x),
                None => ()
            }
        }
    }

    Ok(archives)
}

pub fn read_metadata(path: &Path) -> Result<Option<ArchiveMetadata>> {

    let archive_file_path_string = path.file_name_as_string()?;
    let parts: Vec<&str> = archive_file_path_string.split(".").collect::<Vec<&str>>();

    if parts.len() != 4 {
        return Ok(None);
    }

    let prefix = parts[0];
    let timestamp = parts[2];
    let extension = parts[3];

    if extension != "backup" {
        return Ok(None);
    }

    Ok(match timestamp.parse::<i64>() {
        Ok(epoch) => {

            let archive_date = Utc.timestamp(epoch, 0);

            Some(ArchiveMetadata {
                full_path: path.to_path_buf(),
                archive_date,
                prefix: prefix.to_string(),
            })
        },
        Err(err) => None
    })
}

#[derive(Debug)]
pub struct ArchiveMetadata {
    pub prefix: String,
    pub archive_date: DateTime<Utc>,
    pub full_path: PathBuf,
}

pub enum ArchiveType {
    DockerVolumes
}

pub fn parse_archive_type(archive_type_string: &str) -> Result<ArchiveType> {
    match archive_type_string {
        "docker-volumes" => Ok(ArchiveType::DockerVolumes),
        _ => Err(CustomError::from_message(&format!("Backup type not found: {}", archive_type_string)))
    }
}
