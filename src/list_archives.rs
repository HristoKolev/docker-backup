use clap::Arg;

use crate::global::prelude::*;
use crate::archive_helper::{list_archives, ArchiveType, parse_archive_type};

struct ListCommandOptions {
    #[allow(unused)]
    archive_type: ArchiveType,
    prefix: String,
}

fn list_command_options() -> Result<ListCommandOptions> {

    const ARCHIVE_TYPE_VALUE: &str = "archive-type";

    let matches =  cli().command_config(|x| {

        x.arg(Arg::with_name(ARCHIVE_TYPE_VALUE)
            .short("t")
            .long(ARCHIVE_TYPE_VALUE)
            .value_name(ARCHIVE_TYPE_VALUE)
            .help("The type of archive you want to create.")
            .required(true)
            .takes_value(true)
        )
    });

    let archive_type_string = matches.value_of(ARCHIVE_TYPE_VALUE)
        .ok_or_else(|| CustomError::from_message(&format!("No value for: {}", ARCHIVE_TYPE_VALUE)))?;

    let archive_type = parse_archive_type(archive_type_string)?;


    Ok(ListCommandOptions {
        prefix: archive_type_string.to_string(),
        archive_type
    })
}

pub fn list_archive_command() -> Result {

    let options = list_command_options()?;

    let list = list_archives(&options.prefix)?;

    for item in list {
        log!(
            "{} | {} | {}",
            item.full_path.file_name_as_string()?,
            item.prefix,
            item.archive_date.format("%Y-%m-%d %H:%M:%S").to_string()
        );
    }

    Ok(())
}


