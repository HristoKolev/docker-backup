#[macro_use]
extern crate derive_more;

extern crate lettre;
extern crate lettre_email;
extern crate failure;

extern crate serde;
extern crate serde_json;

mod bash_shell;
mod errors;
mod do_try;
mod email;
mod app_config;
mod email_report;

use crate::errors::{GeneralError, handle_error};
use crate::app_config::AppConfig;

fn main() {

    let result = main_result();

    match result {
        Ok(..) => {},
        Err(error) => {
            handle_error(error)
        }
    }
}

pub fn main_result () -> Result<(), GeneralError> {

    let app_config = app_config::read_config()?;

    let result = run_backup(&app_config);

    match result {
        Ok(..) => {},
        Err(err) => {
            email_report::send_report(&app_config, &err)?;
            println!("{:#?}", err);
        }
    }

    Ok(())
}

pub fn run_backup(app_config: &AppConfig) -> Result<(), GeneralError> {

    let ps_result = bash_shell::exec("echo1 `docker ps -a -q`")?.as_result()?;

    do_try::run(|| {

        do_try::run(|| {

            bash_shell::exec(&format!(
                "rsync -a {}/ {}/",
                app_config.docker_config.volumes_path,
                app_config.docker_config.volumes_mirror_path
            ))?.as_result()?;

            bash_shell::exec(&format!(
                "docker pause {}",
                ps_result.stdout
            ))?.as_result()?;

            bash_shell::exec(&format!(
                "rsync -a {}/ {}/",
                app_config.docker_config.volumes_path,
                app_config.docker_config.volumes_mirror_path
            ))?.as_result()?;

            Ok(())

        }).finally(|| {

            bash_shell::exec(&format!(
                "docker unpause {}", ps_result.stdout
            ))?.as_result()?;

            Ok(())
        })?;

        bash_shell::exec(&format!(
            "cd {} && tar -cpf {} --use-compress-program=\"pigz\" ./",
            app_config.docker_config.volumes_mirror_path,
            app_config.docker_config.archive_path
        ))?.as_result()?;

        Ok(())

    }).finally(|| {

        bash_shell::exec(&format!(
            "rm {} -rf",
            app_config.docker_config.volumes_mirror_path
        ))?.as_result()?;

        Ok(())
    })?;

    Ok(())
}
