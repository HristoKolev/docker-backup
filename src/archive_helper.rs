use std::path::{Path, PathBuf};

use uuid::Uuid;
use chrono::Utc;
use chrono::offset::TimeZone;

use crate::global::{do_try};
use crate::global::prelude::*;
use sentry::internals::DateTime;
use crate::app_config::{ArchiveConfig, CustomArchiveConfig};
use time::Duration;

static ARCHIVE_EXTENSION: &str = "backup";

pub struct ArchiveOptions {
    pub prefix: String,
    pub file_path: Option<String>,
    pub no_encryption: bool,
    pub archive_config: ArchiveConfig,
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

pub fn get_daily_archive_path(options: &ArchiveOptions) -> Result<String> {


    let now = app_start_time();

    let daily_folder = Path::new(&options.archive_config.cache_path)
        .combine_with(&now.format("day_%Y_%m_%d").to_string())
        .create_directory()?;

    let archive_file_name = format!(
        "{}.{}.{}.{}",
        prefix,
        now.format("%Y-%m-%d").to_string(),
        now.timestamp().to_string(),
        ARCHIVE_EXTENSION
    );

    let archive_file_path = daily_folder
        .combine_with(&archive_file_name)
        .get_as_string()?;

    Ok(archive_file_path)
}

pub fn list_archives(prefix_option: Option<&str>) -> Result<Vec<ArchiveMetadata>> {

    let mut archives = Vec::new();

    let daily_folders = ::std::fs::read_dir(&app_config.archive_config.cache_path)?;

    for daily_folder_result in daily_folders {

        let daily_folder = daily_folder_result?.path();

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

pub fn read_metadata(path: &Path) -> Result<Option<ArchiveMetadata>> {

    let archive_file_path_string = path.file_name_as_string()?;
    let parts: Vec<&str> = archive_file_path_string.split(".").collect::<Vec<&str>>();

    if parts.len() != 4 {
        return Ok(None);
    }

    let prefix = parts[0];
    let timestamp = parts[2];
    let extension = parts[3];

    if extension != ARCHIVE_EXTENSION {
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

#[derive(Debug)]
pub struct ArchiveMetadata {
    pub prefix: String,
    pub archive_type: ArchiveType,
    pub archive_date: DateTime<Utc>,
    pub full_path: PathBuf,
}

#[derive(Debug)]
pub enum ArchiveType {
    DockerVolumes
}

pub fn parse_archive_type(archive_type_string: &str) -> Result<ArchiveType> {
    match archive_type_string {
        "docker-volumes" => Ok(ArchiveType::DockerVolumes),
        _ => Err(CustomError::user_error(&format!("Archive type not found: {}", archive_type_string)))
    }
}

pub fn clear_cache(prefix: Option<&str>) -> Result {

    log!("Clearing local cache...");

    let list = list_archives(prefix)?;

    for item in list {

        let expiration_time = *app_start_time() - Duration::days(app_config.archive_config.cache_expiry_days);

        if item.archive_date < expiration_time {

            log!("Deleting `{}` ...", item.full_path.get_as_string()?);

            ::std::fs::remove_file(item.full_path)?;
        }
    }

    Ok(())
}

pub trait ArchiveConfigExtensions {

    fn as_config(&self) -> ArchiveConfig;
}

impl ArchiveConfigExtensions for CustomArchiveConfig {
    fn as_config(&self) -> ArchiveConfig {

        let app_config = app_config();

        ArchiveConfig {
            temp_path: self.temp_path.unwrap_or_else(|| app_config.archive_config.temp_path),
            cache_path: self.cache_path.unwrap_or_else(|| app_config.archive_config.cache_path),
            archive_password: self.archive_password.unwrap_or_else(|| app_config.archive_config.archive_password),
            cache_expiry_days: self.cache_expiry_days.unwrap_or_else(|| app_config.archive_config.cache_expiry_days),
        }
    }
}


impl ArchiveConfigExtensions for Option<CustomArchiveConfig> {
    fn as_config(&self) -> ArchiveConfig {

        let app_config = app_config();

        ArchiveConfig {
            temp_path: self.map(|x| x.temp_path).flatten()
                .unwrap_or_else(|| app_config.archive_config.temp_path),
            cache_path: self.map(|x| x.cache_path).flatten()
                .unwrap_or_else(|| app_config.archive_config.cache_path),
            archive_password: self.map(|x| x.archive_password).flatten()
                .unwrap_or_else(|| app_config.archive_config.archive_password),
            cache_expiry_days: self.map(|x| x.cache_expiry_days).flatten()
                .unwrap_or_else(|| app_config.archive_config.cache_expiry_days),
        }
    }
}