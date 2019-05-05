use crate::app_config::AppConfig;
use crate::{bash_shell, do_try};
use crate::errors::*;

pub fn run_backup(app_config: &AppConfig) -> Result<()> {

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
