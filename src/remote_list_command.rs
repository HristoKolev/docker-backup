use clap::Arg;

use crate::global::prelude::*;
use crate::archive_type::*;
use crate::remote_helper::list_remote_archives;
use std::collections::{HashSet};

struct RemoteListCommandOptions {
    archive_type: Option<ArchiveType>,
}

fn remote_list_command_options() -> Result<RemoteListCommandOptions> {

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

    let archive_type = matches.value_of(ARCHIVE_TYPE_VALUE)
        .map_result(|x| parse_archive_type(x))?;

    Ok(RemoteListCommandOptions {
        archive_type
    })
}

pub fn remote_list_archive_command() -> Result {

    let options = remote_list_command_options()?;

    let archives = list_remote_archives(options.archive_type.as_ref())?;

    let mut names = HashSet::new();

    let mut list = Vec::new();

    for archive in &archives {
        if names.insert(archive.archive_metadata.full_path.file_stem_as_string()?) {
            list.push(archive.archive_metadata.clone());
        }
    }

    if list.len() > 0 {

        let max_file_name_length = (&list).into_iter()
            .map_result(|x| x.full_path.file_name_as_string())?
            .order_by_desc(|x| x.len())
            .first().map(|x| x.len())
            .or_error("The Vec does not have any elements.")?;

        let archive_length_length = (&list).into_iter()
            .map(|x| x.archive_type.to_string())
            .order_by_desc(|x| x.len())
            .first().map(|x| x.len())
            .or_error("The Vec does not have any elements.")?;

        for item in &list {
            log!(
                "{} | {} | {}",
                item.full_path.file_name_as_string()?.pad_right(max_file_name_length, ' '),
                item.archive_type.to_string().pad_right(archive_length_length, ' '),
                item.archive_date.format("%Y-%m-%d %H:%M:%S").to_string()
            );
        }
    }

    Ok(())
}
