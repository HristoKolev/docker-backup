use clap::Arg;

use crate::global::prelude::*;
use crate::archive_helper::{create_archive, clear_local_cache, ArchiveOptions, get_new_archive_path};
use crate::archive_type::*;

struct CreateCommandOptions {
    archive_type: ArchiveType,
    file_path: Option<String>,
    no_encryption: bool,
}

fn create_command_options() -> Result<CreateCommandOptions> {

    const ARCHIVE_TYPE_VALUE: &str = "archive-type";
    const FILE_VALUE: &str = "file";
    const NO_ENCRYPTION_VALUE: &str = "no-encryption";

    let matches = cli().command_config(|x| {

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

    if let Some(file_path) = file_path {
        if ::std::fs::metadata(file_path).is_ok() {
            return Err(CustomError::user_error(&format!("File `{}` already exists", file_path)));
        }
    }

    let archive_type = parse_archive_type(archive_type_string)?;

    let no_encryption = matches.is_present(NO_ENCRYPTION_VALUE);

    Ok(CreateCommandOptions {
        archive_type,
        file_path: file_path.map(|x| x.to_string()),
        no_encryption,
    })
}

pub fn create_archive_command() -> Result {

    let options = create_command_options()?;

    let func = get_create_archive(&options.archive_type);

    let file_path = match options.file_path {
        Some(x) => x,
        None => get_new_archive_path(&options.archive_type)?
    };

    let archive_options = ArchiveOptions {
        no_encryption: options.no_encryption,
        file_path,
        archive_type: options.archive_type.clone()
    };

    create_archive(archive_options, func)?;

    clear_local_cache(Some(&options.archive_type))?;

    email_report::send_success_report(&options.archive_type)?;

    Ok(())
}


