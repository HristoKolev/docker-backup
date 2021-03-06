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
    pub is_cached_archive: bool,
}

pub struct RestoreArchiveOptions {
    pub file_path: PathBuf,
    pub no_decryption: bool,
    pub archive_type: ArchiveType,
}

pub struct UnpackArchiveOptions {
    pub file_path: PathBuf,
    pub out_path: PathBuf,
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

    let parts = archive_file_path_string.split(".").collect_vec();

    if parts.len() != 5 {
        return Ok(None);
    }

    let archive_type_string = parts[0];
    let archive_type_name = parts[1];
    let timestamp = parts[3];
    let extension = parts[4];

    if extension != ARCHIVE_FILE_EXTENSION {
        return Ok(None);
    }

    Ok(match timestamp.parse::<i64>() {
        Ok(epoch) => {

            let archive_type = match parse_archive_type(&format!("{}.{}", archive_type_string, archive_type_name)) {
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

pub fn create_archive<F>(options: CreateArchiveOptions, func: F) -> Result<Option<ArchiveMetadata>>
    where F: FnOnce(&str, &str) -> Result {

    let archive_config = get_archive_config(&options.archive_type);

    let work_path = Path::new(&archive_config.temp_path)
        .join(&Uuid::new_v4().to_string());

    do_try::run(|| {

        bash_exec!("mkdir -p {0} && chmod 777 {0}", &work_path.get_as_string()?);

        let uncompressed = work_path
            .join("uncompressed-archive")
            .get_as_string()?;

        bash_exec!("mkdir -p {0} && chmod 777 {0}", uncompressed);

        func(&options.archive_type.get_config_name(), &uncompressed)?;

        let compressed = work_path
            .join("compressed-archive.tar.gz")
            .get_as_string()?;

        bash_exec!(
            "cd {} && tar cf - . | pigz -{} > {}",
            uncompressed,
            archive_config.rar_compression_level,
            compressed
        );

        bash_exec!("rm {0} -rf", uncompressed);

        let final_archive = work_path
            .join("final.enc")
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

        if options.is_cached_archive {

            let metadata = read_metadata(Path::new(&options.file_path))?
                .or_error("The archiver somehow did not produce a correct archive.")?;

            Ok(Some(metadata))
        } else {
            Ok(None)
        }

    }).finally(|| {

        bash_exec!( "rm {} -rf", work_path.get_as_string()?);

        Ok(())
    })
}

pub fn restore_archive<F>(options: RestoreArchiveOptions, func: F) -> Result
    where F: FnOnce(&str, &str, &str) -> Result {

    let archive_config = get_archive_config(&options.archive_type);

    let work_path = Path::new(&archive_config.temp_path)
        .join(&Uuid::new_v4().to_string());

    do_try::run(|| {

        bash_exec!("mkdir -p {0} && chmod 777 {0}", &work_path.get_as_string()?);

        let encrypted = options.file_path.get_as_string()?;

        let compressed = work_path
            .join("compressed-archive.tar.gz")
            .get_as_string()?;

        if options.no_decryption {
            bash_exec!("cp {} {}", &encrypted, &compressed);
        } else {
            bash_exec!("gpg --output {} -d {}", &compressed, &encrypted);
        }

        func(&options.archive_type.get_config_name(), &work_path.get_as_string()?, &compressed)?;

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
        .join(&now.format("day_%Y_%m_%d").to_string())
        .create_directory()?;

    let archive_file_name = format!(
        "{}.{}.{}.{}",
        archive_type.to_string(),
        now.format("%Y-%m-%d").to_string(),
        now.timestamp().to_string(),
        ARCHIVE_FILE_EXTENSION
    );

    let archive_file_path = daily_folder
        .join(&archive_file_name);

    Ok(archive_file_path)
}

pub fn list_local_archives(archive_type: Option<&ArchiveType>) -> Result<Vec<ArchiveMetadata>> {

    let archive_types = archive_type
        .map(|x| vec![x.clone()])
        .unwrap_or_else(|| ArchiveType::all());

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

            ::std::fs::remove_file(&archive_metadata.full_path)?;

            let mut daily_directory = archive_metadata.full_path.clone();
            daily_directory.pop();

            let is_empty = !::std::fs::read_dir(&daily_directory)?.has_any();

            if is_empty {

                log!("Deleting daily folder `{}` ...", daily_directory.get_as_string()?);

                ::std::fs::remove_dir(&daily_directory)?;
            }
        }
    }

    Ok(())
}

pub fn unpack_archive(options: UnpackArchiveOptions) -> Result {

    let archive_config = get_archive_config(&options.archive_type);

    let work_path = Path::new(&archive_config.temp_path)
        .join(&Uuid::new_v4().to_string());

    do_try::run(|| {

        bash_exec!("mkdir -p {0} && chmod 777 {0}", &work_path.get_as_string()?);

        let encrypted = options.file_path.get_as_string()?;

        let compressed = work_path
            .join("compressed-archive.tar.gz")
            .get_as_string()?;

        if options.no_decryption {
            bash_exec!("cp {} {}", &encrypted, &compressed);
        } else {
            bash_exec!("gpg --output {} -d {}", &compressed, &encrypted);
        }

        let out_path = options.out_path.get_as_string()?;

        bash_exec!(
            "mkdir -p {0} && cd {0} && tar -xf {1}",
            &out_path,
            &compressed
        );

        Ok(())

    }).finally(|| {

        bash_exec!( "rm {} -rf", work_path.get_as_string()?);

        Ok(())
    })
}
