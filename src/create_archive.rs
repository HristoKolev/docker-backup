use clap::Arg;

use crate::global::{do_try, app_config};
use crate::global::prelude::*;
use crate::archive_helper::{ArchiveType, parse_archive_type, create_archive};

struct CreateCommandOptions {
    #[allow(unused)]
    archive_type: ArchiveType,
    prefix: String,
    file_path: Option<String>,
    no_encryption: bool,
}

fn create_command_options() -> Result<CreateCommandOptions> {

    const ARCHIVE_TYPE_VALUE: &str = "archive-type";
    const FILE_VALUE: &str = "file";
    const NO_ENCRYPTION_VALUE: &str = "no-encryption";

    let matches =  cli().command_config(|x| {

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
        prefix: archive_type_string.to_string(),
        file_path: file_path.map(|x| x.to_string()),
        no_encryption,
    })
}

pub fn create_archive_command() -> Result {

    let options = create_command_options()?;

    create_archive(&options.prefix, |work_path| {

        let app_config = app_config();

        let ps_result = bash_exec!("echo `docker ps -a -q`");

        do_try::run(|| {

            bash_exec!(
                "rsync -a {}/ {}/",
                app_config.docker_config.volumes_path,
                work_path
            );

            bash_exec!("docker pause {}", ps_result.stdout);

            bash_exec!(
                "rsync -a {}/ {}/",
                app_config.docker_config.volumes_path,
                work_path
            );

            Ok(())
        }).finally(|| {

            bash_exec!("docker unpause {}", ps_result.stdout);

            Ok(())
        })?;

        Ok(())
    })?;

    Ok(())
}


