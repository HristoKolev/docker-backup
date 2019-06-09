use std::path::{Path, PathBuf};

use uuid::Uuid;
use chrono::{Utc, DateTime};
use chrono::offset::TimeZone;
use time::Duration;

use crate::global::{do_try};
use crate::global::prelude::*;
use crate::archive_type::*;

static ARCHIVE_FILE_EXTENSION: &str = "backup";

pub struct ArchiveOptions {
    pub prefix: String,
    pub file_path: Option<String>,
    pub no_encryption: bool,
    pub archive_config: ArchiveConfig,
}

#[derive(Debug)]
pub struct ArchiveMetadata {
    pub prefix: String,
    pub archive_type: ArchiveType,
    pub archive_date: DateTime<Utc>,
    pub full_path: PathBuf,
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
                prefix: prefix.to_string(),
                archive_type
            })
        },
        Err(_) => None
    })
}

pub fn create_archive<F>(options: ArchiveOptions, func: F) -> Result
    where F: FnOnce(&str) -> Result {

    let work_path = Path::new(&options.archive_config.temp_path)
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
                r#"echo "{0}" | gpg --symmetric --batch --passphrase-fd 0 --cipher-algo AES256 --output {1} {2}"#,
                &options.archive_config.archive_password,
                &final_archive,
                &compressed
            );

            bash_exec!("rm {0} -f", compressed);
        }

        let archive_file_path = match options.file_path {
            Some(x) => x,
            None => get_daily_archive_path(&options)?
        };

        bash_exec!("mv {} {}", &final_archive, &archive_file_path);

        Ok(())
    }).finally(|| {

        bash_exec!( "rm {} -rf", work_path.get_as_string()?);

        Ok(())
    })?;

    Ok(())
}

fn get_daily_archive_path(options: &ArchiveOptions) -> Result<String> {

    let now = app_start_time();

    let daily_folder = Path::new(&options.archive_config.cache_path)
        .combine_with(&now.format("day_%Y_%m_%d").to_string())
        .create_directory()?;

    let archive_file_name = format!(
        "{}.{}.{}.{}",
        options.prefix,
        now.format("%Y-%m-%d").to_string(),
        now.timestamp().to_string(),
        ARCHIVE_FILE_EXTENSION
    );

    let archive_file_path = daily_folder
        .combine_with(&archive_file_name)
        .get_as_string()?;

    Ok(archive_file_path)
}

pub fn list_local_cache_archives(prefix_option: Option<&str>) -> Result<Vec<ArchiveMetadata>> {

    let mut archives = Vec::new();

    let daily_folders = match prefix_option {
        Some(prefix) => {

            let archive_type = parse_archive_type(prefix)?;

            let custom_config = get_archive_config(archive_type);

            let mut dirs = Vec::new();

            for item in ::std::fs::read_dir(&custom_config.cache_path)? {
                dirs.push(item?);
            }

            dirs
        },
        None => {

            let mut dirs = Vec::new();

            for archive_type in ArchiveType::all() {

                let custom_config = get_archive_config(archive_type);

                for item in ::std::fs::read_dir(&custom_config.cache_path)? {
                    dirs.push(item?);
                }
            }

            dirs
        }
    };

    for daily_folder_result in daily_folders {

        let daily_folder = daily_folder_result.path();

        for archive_file_result in ::std::fs::read_dir(daily_folder)? {

            let archive_path = archive_file_result?.path();

            let metadata = read_metadata(&archive_path)?;

            match metadata {
                Some(x) => {
                    if let Some(prefix) = prefix_option {
                        if &x.prefix == prefix {
                            archives.push(x)
                        }
                    } else {
                        archives.push(x)
                    }
                },
                None => ()
            }
        }
    }

    Ok(archives)
}

pub fn clear_local_cache(prefix: Option<&str>) -> Result {

    log!("Clearing local cache...");

    let list = list_local_cache_archives(prefix)?;

    for item in list {

        let archive_config = get_archive_config(item.archive_type);

        let expiration_time = *app_start_time() - Duration::days(archive_config.cache_expiry_days);

        if item.archive_date < expiration_time {

            log!("Deleting `{}` ...", item.full_path.get_as_string()?);

            ::std::fs::remove_file(item.full_path)?;
        }
    }

    Ok(())
}
