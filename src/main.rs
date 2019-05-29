#![forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod global;
mod run_backup;

use crate::global::prelude::*;
use crate::run_backup::{create_archive, list_archives};
use crate::global::{do_try, app_config, logger};
use crate::global::cli::command_config;
use clap::Arg;

fn main() {

    global::initialize();

    main_result().crash_on_error();
}

fn main_result() -> Result {

    let matches = command_config("create", |x| {

        x.arg(Arg::with_name("backup-type")
            .short("t")
            .long("backup-type")
            .value_name("backup-type")
            .help("The type of backup you want to create.")
            .required(true)
            .takes_value(true)
        )
    });

    let config = matches.value_of("backup-type");

    println!("{}", config.unwrap());


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
