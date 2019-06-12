use clap::Arg;

use crate::global::prelude::*;
use crate::archive_type::*;

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

    Ok(())
}