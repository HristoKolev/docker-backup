
use clap::Arg;

use crate::global::prelude::*;
use crate::archive_type::*;
use std::path::{Path, PathBuf};
use crate::archive_helper::{UnpackArchiveOptions, unpack_archive};

struct UnpackCommandOptions {
    archive_type: ArchiveType,
    file_path: PathBuf,
    out_path: PathBuf,
    no_decryption: bool,
}

fn unpack_command_options() -> Result<UnpackCommandOptions> {

    const ARCHIVE_TYPE_VALUE: &str = "archive-type";
    const FILE_VALUE: &str = "file";
    const OUT_PATH_VALUE: &str = "out-path";
    const NO_DECRYPTION_VALUE: &str = "no-decryption";

    let matches = cli().command_config(|x| {

        x.arg(Arg::with_name(ARCHIVE_TYPE_VALUE)
            .short("t")
            .long(ARCHIVE_TYPE_VALUE)
            .value_name(ARCHIVE_TYPE_VALUE)
            .help("The type of archive you want to restore.")
            .required(true)
            .takes_value(true)
        ).arg(Arg::with_name(FILE_VALUE)
            .short("f")
            .long(FILE_VALUE)
            .value_name(FILE_VALUE)
            .help("The file path.")
            .required(true)
            .takes_value(true)
        ).arg(Arg::with_name(OUT_PATH_VALUE)
            .short("o")
            .long(OUT_PATH_VALUE)
            .value_name(OUT_PATH_VALUE)
            .help("The output path.")
            .required(true)
            .takes_value(true)
        ).arg(Arg::with_name(NO_DECRYPTION_VALUE)
            .short("n")
            .long(NO_DECRYPTION_VALUE)
            .value_name(NO_DECRYPTION_VALUE)
            .help("Do not decrypt the archive.")
            .required(false)
            .takes_value(false)
        )
    });

    let file_path = matches.value_of(FILE_VALUE);

    if let Some(file_path) = file_path {
        if ::std::fs::metadata(file_path).is_err() {
            return Err(CustomError::user_error(&format!("File `{}` does not exists", file_path)));
        }
    }

    let archive_type_string = matches.value_of(ARCHIVE_TYPE_VALUE)
        .ok_or_else(|| CustomError::from_message(&format!("No value for: {}", ARCHIVE_TYPE_VALUE)))?;

    let archive_type = parse_archive_type(archive_type_string)?;

    let file_path = matches.value_of(FILE_VALUE)
        .ok_or_else(|| CustomError::from_message(&format!("No value for: {}", FILE_VALUE)))?;

    let out_path = matches.value_of(OUT_PATH_VALUE)
        .ok_or_else(|| CustomError::from_message(&format!("No value for: {}", OUT_PATH_VALUE)))?;

    let no_decryption = matches.is_present(NO_DECRYPTION_VALUE);

    Ok(UnpackCommandOptions {
        archive_type,
        file_path: Path::new(file_path).to_path_buf(),
        out_path: Path::new(out_path).to_path_buf(),
        no_decryption,
    })
}

pub fn unpack_archive_command() -> Result {

    let options = unpack_command_options()?;

    let archive_options = UnpackArchiveOptions {
        no_decryption: options.no_decryption,
        file_path: options.file_path.clone(),
        out_path: options.out_path.clone(),
        archive_type: options.archive_type.clone()
    };

    unpack_archive(archive_options)?;

    Ok(())
}
