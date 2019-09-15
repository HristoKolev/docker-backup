use clap::Arg;

use crate::global::prelude::*;
use crate::archive_type::*;
use crate::remote_helper::{clear_remote_cache};

struct ClearRemoteCacheCommandOptions {
    archive_type: Option<ArchiveType>,
}

fn clear_remote_cache_command_options() -> Result<ClearRemoteCacheCommandOptions> {

    const ARCHIVE_TYPE_VALUE: &str = "archive-type";

    let matches = cli().command_config(|x| {

        x.arg(Arg::with_name(ARCHIVE_TYPE_VALUE)
            .short("t")
            .long(ARCHIVE_TYPE_VALUE)
            .value_name(ARCHIVE_TYPE_VALUE)
            .help("The type of archive you want to clear the remote cache of.")
            .required(false)
            .takes_value(true)
        )
    });

    let archive_type = matches.value_of(ARCHIVE_TYPE_VALUE)
        .map_result(|x| parse_archive_type(x))?;

    Ok(ClearRemoteCacheCommandOptions {
        archive_type
    })
}

pub fn clear_remote_cache_command() -> Result {

    let options = clear_remote_cache_command_options()?;

    let archive_types = options.archive_type
        .map(|x| vec![x.clone()])
        .unwrap_or_else(|| ArchiveType::all());

    let mut results = Vec::new();

    for archive_type in archive_types {

        results.push(clear_remote_cache(&archive_type));
    }

    for result in results {
        result?;
    }

    Ok(())
}
