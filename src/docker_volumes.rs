use crate::global::prelude::*;
use crate::global::{do_try, app_config};

pub fn create_docker_volumes_archive(config_name: &str, work_path: &str) -> Result {

    let config = app_config().docker_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .ok_or_else(|| CustomError::from_message("`DockerVolumes` archiving is not configured."))?;

    let ps_result = bash_exec!("echo `docker ps -a -q`");

    do_try::run(|| {

        bash_exec!("rsync -a {}/ {}/", config.volumes_path, work_path);

        bash_exec!("docker pause {}", ps_result.stdout);

        bash_exec!("rsync -a {}/ {}/", config.volumes_path, work_path);

        Ok(())
    }).finally(|| {

        bash_exec!("docker unpause {}", ps_result.stdout);

        Ok(())
    })?;

    Ok(())
}

pub fn restore_docker_volumes_archive(config_name: &str, _work_path: &str, compressed: &str) -> Result {

    let config = app_config().docker_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .ok_or_else(|| CustomError::from_message("`DockerVolumes` archiving is not configured."))?;

    bash_exec!("systemctl stop docker");

    bash_exec!("rm {0} -rf && mkdir -p {0}", &config.volumes_path);

    bash_exec!("cd {} && tar -xf {} --use-compress-program=pigz", &config.volumes_path, &compressed);

    bash_exec!("systemctl start docker");

    Ok(())
}