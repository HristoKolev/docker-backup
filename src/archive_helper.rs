use std::path::{Path, PathBuf};

use uuid::Uuid;
use chrono::{Utc, DateTime};
use chrono::offset::TimeZone;
use time::Duration;

use crate::global::{do_try};
use crate::global::prelude::*;
use crate::archive_type::*;

static ARCHIVE_FILE_EXTENSION: &str = "backup";

pub struct CreateArchiveOptions {
    pub file_path: PathBuf,
    pub no_encryption: bool,
    pub archive_type: ArchiveType,
}

pub struct RestoreArchiveOptions {
    pub file_path: PathBuf,
    pub no_decryption: bool,
    pub archive_type: ArchiveType,
}

#[derive(Debug, Clone)]
pub struct ArchiveMetadata {
    pub archive_type: ArchiveType,
    pub archive_date: DateTime<Utc>,
    pub full_path: PathBuf,
}

pub fn read_metadata(path: &Path) -> Result<Option<ArchiveMetadata>> {

    let archive_file_path_string = path.file_name_as_string()?;
    let parts: Vec<&str> = archive_file_path_string.split(".").collect();

    if parts.len() != 4 {
        return Ok(None);
    }

    let prefix = parts[0];
    let timestamp = parts[2];
    let extension = parts[3];

    if extension != ARCHIVE_FILE_EXTENSION {
        return Ok(None);
    }

    Ok(match timestamp.parse::<i64>() {
        Ok(epoch) => {

            let archive_type = match parse_archive_type(prefix) {
                Ok(x) => x,
                Err(_) => return Ok(None)
            };

            Some(ArchiveMetadata {
                full_path: path.to_path_buf(),
                archive_date: Utc.timestamp(epoch, 0),
                archive_type
            })
        },
        Err(_) => None
    })
}

pub fn create_archive<F>(options: CreateArchiveOptions, func: F) -> Result<ArchiveMetadata>
    where F: FnOnce(&str) -> Result {

    let archive_config = get_archive_config(&options.archive_type);

    let work_path = Path::new(&archive_config.temp_path)
        .combine_with(&Uuid::new_v4().to_string());

    do_try::run(|| {

        bash_exec!("mkdir -p {0} && chmod 777 {0}", &work_path.get_as_string()?);

        let uncompressed = work_path
            .combine_with("uncompressed-archive")
            .get_as_string()?;

        bash_exec!("mkdir -p {0} && chmod 777 {0}", uncompressed);

        func(&uncompressed)?;

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

        if options.no_encryption {

            bash_exec!("mv {} {}", &compressed, &final_archive);
        } else {

            bash_exec!(
                "gpg -e --batch -r {0} --output {1} {2}",
                &archive_config.gpg_key_name,
                &final_archive,
                &compressed
            );

            bash_exec!("rm {0} -f", compressed);
        }

        bash_exec!("mv {} {}", &final_archive, &options.file_path.get_as_string()?);

        let metadata = read_metadata(Path::new(&options.file_path))?
            .ok_or_else(|| CustomError::from_message("The archiver somehow did not produce a correct archive."))?;

        Ok(metadata)

    }).finally(|| {

        bash_exec!( "rm {} -rf", work_path.get_as_string()?);

        Ok(())
    })
}

pub fn restore_archive<F>(options: RestoreArchiveOptions, func: F) -> Result
    where F: FnOnce(&str, &str) -> Result {

    let archive_config = get_archive_config(&options.archive_type);

    let work_path = Path::new(&archive_config.temp_path)
        .combine_with(&Uuid::new_v4().to_string());

    do_try::run(|| {

        bash_exec!("mkdir -p {0} && chmod 777 {0}", &work_path.get_as_string()?);

        let encrypted = options.file_path.get_as_string()?;

        let compressed = work_path
            .combine_with("compressed-archive.tar.gz")
            .get_as_string()?;

        if options.no_decryption {
            bash_exec!("cp {} {}", &encrypted, &compressed);
        } else {
            bash_exec!("gpg --output {} -d {}", &compressed, &encrypted);
        }

        func(&work_path.get_as_string()?, &compressed)?;

        Ok(())

    }).finally(|| {

        bash_exec!( "rm {} -rf", work_path.get_as_string()?);

        Ok(())
    })
}

pub fn get_new_archive_path(archive_type: &ArchiveType) -> Result<PathBuf> {

    let archive_config = get_archive_config(archive_type);

    let now = app_start_time();

    let daily_folder = Path::new(&archive_config.cache_path)
        .combine_with(&now.format("day_%Y_%m_%d").to_string())
        .create_directory()?;

    let archive_file_name = format!(
        "{}.{}.{}.{}",
        archive_type.to_string(),
        now.format("%Y-%m-%d").to_string(),
        now.timestamp().to_string(),
        ARCHIVE_FILE_EXTENSION
    );

    let archive_file_path = daily_folder
        .combine_with(&archive_file_name);

    Ok(archive_file_path)
}

pub fn list_local_archives(archive_type: Option<&ArchiveType>) -> Result<Vec<ArchiveMetadata>> {

    let archive_types = match archive_type {
        Some(x) => vec![x.clone()],
        None => ArchiveType::all(),
    };

    let mut archives = Vec::new();

    for archive_type in archive_types {

        let custom_config = get_archive_config(&archive_type);

        let cache_path = Path::new(&custom_config.cache_path);

        if !cache_path.exists() {
            continue;
        }

        for item in ::std::fs::read_dir(cache_path)? {

            let daily_folder = item?.path();

            for archive_file_result in ::std::fs::read_dir(daily_folder)? {

                let archive_path = archive_file_result?.path();

                let metadata = read_metadata(&archive_path)?;

                match metadata {
                    Some(x) => {
                        if x.archive_type == archive_type {
                            archives.push(x);
                        }
                    },
                    None => ()
                }
            }
        }
    }

    Ok(archives)
}

pub fn clear_local_cache(archive_type: Option<&ArchiveType>) -> Result {

    log!("Clearing local cache...");

    let list = list_local_archives(archive_type)?;

    for archive_metadata in list {

        let archive_config = get_archive_config(&archive_metadata.archive_type);

        let expiration_time = *app_start_time() - Duration::days(archive_config.cache_expiry_days);

        if archive_metadata.archive_date < expiration_time {

            log!("Deleting `{}` ...", archive_metadata.full_path.get_as_string()?);

            ::std::fs::remove_file(archive_metadata.full_path)?;
        }
    }

    Ok(())
}
