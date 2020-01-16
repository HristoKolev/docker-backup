use crate::global::prelude::*;
use crate::global::{do_try};
use crate::global::file_lock::wait_for_lock;

pub fn create_docker_volumes_archive(config_name: &str, work_path: &str) -> Result {

    let config = app_config().docker_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .or_error("`DockerVolumes` archiving is not configured.")?;

    let app_config = app_config();

    let archive_config = config.archive_config.as_ref()
        .unwrap_or_else(|| &app_config.archive_config);

    let lock_name = &format!("{}/docker-volumes.lock", &archive_config.temp_path);

    log!("Acquiring lock - `{}` ...", lock_name);
    let _lock = wait_for_lock(lock_name);
    log!("Lock `{}` acquired.", lock_name);

    let ps_result = bash_exec!(r##"docker ps --filter="status=running" -q"##);
    let container_ids = ps_result.stdout.replace("\n", " ");

    do_try::run(|| {

        copy_files(&config.volumes_path, work_path)?;

        bash_exec!("docker pause {}", container_ids);

        copy_files(&config.volumes_path, work_path)?;

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

    bash_exec!("cd {} && tar -xf {}", &config.volumes_path, &compressed);

    bash_exec!("systemctl start docker");

    Ok(())
}


fn copy_files(source: &str, destination: &str) -> Result {

    let mut command_result;
    let mut i = 0;

    let max_tries = 10;

    loop {

        command_result = bash_shell::exec(
            &format!(
                r##"rsync -a --delete --filter="dir-merge,- .backupignore" {}/ {}/"##,
                source,
                destination
            )
        )?;

        match command_result.as_result_ref() {
            Ok(_) => return Ok(()),
            Err(err) => {

                if !command_result.stderr.contains("vanished") {

                    return Err(err);
                }

                if i == max_tries {

                    return Err(CustomError::from_message("The files keep vanishing."))
                }
            }
        }

        i +=1;
    }
}
