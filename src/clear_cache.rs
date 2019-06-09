use clap::Arg;

use crate::global::prelude::*;
use crate::archive_helper::{clear_cache};
use crate::archive_type::*;

struct ClearCacheCommandOptions {
    #[allow(unused)]
    archive_type: Option<ArchiveType>,
    prefix: Option<String>,
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

    let archive_type_string = matches.value_of(ARCHIVE_TYPE_VALUE);

    let archive_type = match archive_type_string {
        Some(xx) => Some(parse_archive_type(xx)?),
        None => None
    };

    Ok(ClearCacheCommandOptions {
        prefix: archive_type_string.map(|x| x.to_string()),
        archive_type
    })
}

pub fn clear_cache_command() -> Result {

    let options = clear_cache_command_options()?;

    clear_cache(options.prefix.as_ref().map(String::as_ref))?;

    Ok(())
}