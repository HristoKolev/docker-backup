use std::path::{Path, PathBuf};

use uuid::Uuid;
use chrono::Utc;
use chrono::offset::TimeZone;
use clap::Arg;

use crate::global::{do_try, app_config};
use crate::global::prelude::*;
use sentry::internals::DateTime;
use crate::archive_helper::{ArchiveType, parse_archive_type};

struct CreateCommandOptions {
    archive_type: ArchiveType,
    file_path: Option<String>,
    no_encryption: bool,
}

fn create_command_options() -> Result<CreateCommandOptions> {

    const ARCHIVE_TYPE_VALUE: &str = "backup-type";
    const FILE_VALUE: &str = "file";
    const NO_ENCRYPTION_VALUE: &str = "no-encryption";

    let matches =  cli().command_config("create", |x| {

        x.arg(Arg::with_name(ARCHIVE_TYPE_VALUE)
            .short("t")
            .long(ARCHIVE_TYPE_VALUE)
            .value_name(ARCHIVE_TYPE_VALUE)
            .help("The type of archive you want to create.")
            .required(true)
            .takes_value(true)
        ).arg(Arg::with_name(FILE_VALUE)
            .short("f")
            .long(FILE_VALUE)
            .value_name(FILE_VALUE)
            .help("The file path.")
            .required(false)
            .takes_value(true)
        ).arg(Arg::with_name(NO_ENCRYPTION_VALUE)
            .short("n")
            .long(NO_ENCRYPTION_VALUE)
            .value_name(NO_ENCRYPTION_VALUE)
            .help("Do not encrypt the archive.")
            .required(false)
            .takes_value(false)
        )
    });

    let archive_type_string = matches.value_of(ARCHIVE_TYPE_VALUE)
        .ok_or_else(|| CustomError::from_message(&format!("No value for: {}", ARCHIVE_TYPE_VALUE)))?;

    let file_path = matches.value_of(FILE_VALUE);

    let archive_type = parse_archive_type(archive_type_string)?;

    let no_encryption = matches.is_present(NO_ENCRYPTION_VALUE);

    Ok(CreateCommandOptions {
        archive_type,
        file_path: file_path.map(|x| x.to_string()),
        no_encryption,
    })
}

pub fn create_archive() -> Result {

    let options = create_command_options()?;

    Ok(())
}


