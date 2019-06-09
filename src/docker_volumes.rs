use crate::global::prelude::*;
use crate::global::{do_try, app_config};

pub fn create_docker_volumes_archive(work_path: &str) -> Result {

    let config = app_config().docker_config.clone()
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
