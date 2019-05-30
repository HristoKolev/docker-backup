use clap::{App, ArgMatches, Arg};
use std::ffi::OsString;

use crate::global::prelude::*;

pub fn run() {
}

enum BackupType {
    DockerVolumes
}

struct  CreateCommandOptions {
    backup_type: BackupType,
    file_path: Option<String>,
    no_encryption: bool,
}

fn parse_backup_type(backup_type_string: &str) -> Result<BackupType> {
     match backup_type_string {
         "docker-volumes" => Ok(BackupType::DockerVolumes),
         _ => Err(CustomError::from_message(&format!("Backup type not found: {}", backup_type_string)))
     }
}

fn create_command_options() -> Result<CreateCommandOptions> {

    const BACKUP_TYPE_VALUE: &str = "backup-type";
    const FILE_VALUE: &str = "file";
    const NO_ENCRYPTION_VALUE: &str = "no-encryption";

    let matches = command_config("create", |x| {

        x.arg(Arg::with_name(BACKUP_TYPE_VALUE)
            .short("t")
            .long(BACKUP_TYPE_VALUE)
            .value_name(BACKUP_TYPE_VALUE)
            .help("The type of backup you want to create.")
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

    let backup_type_string = matches.value_of(BACKUP_TYPE_VALUE)
        .ok_or_else(|| CustomError::from_message(&format!("No value for: {}", BACKUP_TYPE_VALUE)))?;

    let file_path = matches.value_of(FILE_VALUE);

    let backup_type = parse_backup_type(backup_type_string)?;

    let no_encryption = matches.is_present(NO_ENCRYPTION_VALUE);

    Ok(CreateCommandOptions {
        backup_type,
        file_path: file_path.map(|x| x.to_string()),
        no_encryption,
    })
}

fn command_config<F>(command_name: &str, f: F) -> ArgMatches
    where F: for<'a, 'b> FnOnce(App<'a, 'b>) -> App<'a, 'b> {

    let mut matches = App::new(format!("XDXD Backup - {}", command_name))
        .version("1.0")
        .author("Hristo Kolev")
        .about("Backs things up.");

    matches = f(matches);

    let mut i = 0;
    let args = ::std::env::args_os().filter(|x| {

        let result = i != 1;

        i += 1;

        result
    }).collect::<Vec<OsString>>();

    matches.get_matches_from(args)
}
