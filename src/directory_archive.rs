use crate::global::prelude::*;

pub fn create_directory_archive(config_name: &str, work_path: &str) -> Result {

    let config = app_config().directory_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .or_error("`Directory` archiving is not configured.")?;

    bash_exec!("mkdir -p {0}", &config.directory_path);

    bash_exec!(r##"rsync -a --delete --filter="dir-merge,- .backupignore" {}/ {}/"##, &config.directory_path, work_path);

    Ok(())
}

pub fn restore_directory_archive(config_name: &str, _work_path: &str, compressed: &str) -> Result {

    let config = app_config().directory_config.as_ref()
        .and_then(|x| x.get(config_name).cloned())
        .or_error("`Directory` archiving is not configured.")?;

    bash_exec!("rm {0} -rf && mkdir -p {0}", &config.directory_path);

    bash_exec!(
        "cd {} && unrar x -idq {} ./",
        &config.directory_path,
        &compressed
    );

    Ok(())
}
