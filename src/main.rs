#![forbid(unsafe_code)]

#[macro_use]
mod global;
mod archive_helper;
mod remote_helper;
mod archive_type;
mod docker_volumes;
mod create_archive;
mod list_archives;
mod clear_cache;
mod upload;

use crate::global::prelude::*;
use crate::global::errors::CustomErrorKind;

use crate::create_archive::create_archive_command;
use crate::list_archives::list_archive_command;
use crate::clear_cache::clear_cache_command;
use crate::upload::upload_command;

fn main() {
    global::initialize();
    main_result().crash_on_error();
}

fn main_result() -> Result {

    cli().register_command("create", create_archive_command)?;
    cli().register_command("list", list_archive_command)?;
    cli().register_command("clear-cache", clear_cache_command)?;
    cli().register_command("upload", upload_command)?;

    match cli().run() {
        Err(err) => {
            if let CustomErrorKind::UserError(message) = err.kind {
                log!("Error: {}", message);
                ::std::process::exit(1);
            } else {
                return Err(err);
            }
        },
        Ok(_) => ()
    };

    Ok(())
}
