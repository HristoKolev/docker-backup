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
mod clear_remote_cache;
mod restore_archive;
mod config;
mod unpack;
mod directory_archive;
mod kvm_machine;

use crate::global::prelude::*;
use crate::global::errors::CustomErrorKind;

use crate::create_archive::create_archive_command;
use crate::list_archives::list_archive_command;
use crate::clear_cache::clear_cache_command;
use crate::upload::upload_command;
use crate::clear_remote_cache::clear_remote_cache_command;
use crate::restore_archive::restore_archive_command;
use crate::config::config_command;
use crate::unpack::unpack_archive_command;

fn main() {

    global::initialize();

    main_result().crash_on_error();
}

fn main_result() -> Result {

    cli().register_command("create", Box::new(create_archive_command))?;
    cli().register_command("list", Box::new(list_archive_command))?;
    cli().register_command("clear-cache", Box::new(clear_cache_command))?;
    cli().register_command("upload", Box::new(upload_command))?;
    cli().register_command("clear-remote-cache", Box::new(clear_remote_cache_command))?;
    cli().register_command("restore", Box::new(restore_archive_command))?;
    cli().register_command("unpack", Box::new(unpack_archive_command))?;
    cli().register_command("config", Box::new(config_command))?;

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
