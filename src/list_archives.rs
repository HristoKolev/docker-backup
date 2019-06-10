use clap::Arg;

use crate::global::prelude::*;
use crate::archive_helper::{list_local_archives};
use crate::archive_type::*;

struct ListCommandOptions {
    archive_type: Option<ArchiveType>,
}

fn list_command_options() -> Result<ListCommandOptions> {

    const ARCHIVE_TYPE_VALUE: &str = "archive-type";

    let matches =  cli().command_config(|x| {

        x.arg(Arg::with_name(ARCHIVE_TYPE_VALUE)
            .short("t")
            .long(ARCHIVE_TYPE_VALUE)
            .value_name(ARCHIVE_TYPE_VALUE)
            .help("The type of archive you want to list.")
            .required(false)
            .takes_value(true)
        )
    });

    let archive_type_string = matches.value_of(ARCHIVE_TYPE_VALUE);

    let archive_type = match archive_type_string {
        Some(xx) => Some(parse_archive_type(xx)?),
        None => None
    };

    Ok(ListCommandOptions {
        archive_type
    })
}

pub fn list_archive_command() -> Result {

    let options = list_command_options()?;

    let list = list_local_archives(options.archive_type.as_ref())?;

    for item in list {
        log!(
            "{} | {} | {}",
            item.full_path.file_name_as_string()?,
            item.archive_type.to_string(),
            item.archive_date.format("%Y-%m-%d %H:%M:%S").to_string()
        );
    }

    Ok(())
}


