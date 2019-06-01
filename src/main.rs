#![forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod global;
mod archive_helper;
mod create_archive;

use crate::global::prelude::*;
use crate::create_archive::{create_archive};
use crate::global::{do_try, app_config, logger};
use clap::Arg;
use std::ffi::{OsStr, OsString};
use std::collections::HashMap;

fn main() {

    global::initialize();

    main_result().crash_on_error();
}

fn main_result() -> Result {

    cli().register_command("create", create_archive)?;

    cli().run()?;


//    create_archive("docker-volumes", |work_path| {
//
//        let app_config = app_config();
//
//        let ps_result = bash_exec!("echo `docker ps -a -q`");
//
//        do_try::run(|| {
//
//            bash_exec!(
//                "rsync -a {}/ {}/",
//                app_config.docker_config.volumes_path,
//                work_path
//            );
//
//            bash_exec!("docker pause {}", ps_result.stdout);
//
//            bash_exec!(
//                "rsync -a {}/ {}/",
//                app_config.docker_config.volumes_path,
//                work_path
//            );
//
//            Ok(())
//        }).finally(|| {
//
//            bash_exec!("docker unpause {}", ps_result.stdout);
//
//            Ok(())
//        })?;
//
//        Ok(())
//    })?;

    Ok(())
}
