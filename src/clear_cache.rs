use clap::Arg;

use crate::global::prelude::*;
use crate::archive_helper::{clear_local_cache};
use crate::archive_type::*;

struct ClearCacheCommandOptions {
    archive_type: Option<ArchiveType>,
}

fn clear_cache_command_options() -> Result<ClearCacheCommandOptions> {

    const ARCHIVE_TYPE_VALUE: &str = "archive-type";

    let matches =  cli().command_config(|x| {

        x.arg(Arg::with_name(ARCHIVE_TYPE_VALUE)
            .short("t")
            .long(ARCHIVE_TYPE_VALUE)
            .value_name(ARCHIVE_TYPE_VALUE)
            .help("The type of archive you want to clear the cache of.")
            .required(false)
            .takes_value(true)
        )
    });

    let archive_type = matches.value_of(ARCHIVE_TYPE_VALUE)
        .map_result(|x| parse_archive_type(x))?;

    Ok(ClearCacheCommandOptions {
        archive_type
    })
}

pub fn clear_cache_command() -> Result {

    let options = clear_cache_command_options()?;

    clear_local_cache(options.archive_type.as_ref())?;

    Ok(())
}