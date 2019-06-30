use crate::global::prelude::*;
use crate::global::{do_try};

pub fn create_docker_volumes_archive(config_name: &str, work_path: &str) -> Result {

    let config = app_config().docker_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .or_error("`DockerVolumes` archiving is not configured.")?;

    let ps_result = bash_exec!(r##"docker ps --filter="status=running" -q"##);

    let container_ids = ps_result.stdout.replace("\n", " ");

    do_try::run(|| {

        bash_exec!("rsync -a --delete {}/ {}/", config.volumes_path, work_path);

        bash_exec!("docker pause {}", container_ids);

        bash_exec!("rsync -a --delete {}/ {}/", config.volumes_path, work_path);

        Ok(())
    }).finally(|| {

        bash_exec!("docker unpause {}", container_ids);

        Ok(())
    })?;

    Ok(())
}

pub fn restore_docker_volumes_archive(config_name: &str, _work_path: &str, compressed: &str) -> Result {

    let config = app_config().docker_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .or_error("`DockerVolumes` archiving is not configured.")?;

    bash_exec!("systemctl stop docker");

    bash_exec!("rm {0} -rf && mkdir -p {0}", &config.volumes_path);

    bash_exec!(
        "cd {} && unrar x -idq {} ./",
        &config.volumes_path,
        &compressed
    );

    bash_exec!("systemctl start docker");

    Ok(())
}
