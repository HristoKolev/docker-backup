use std::path::Path;

use uuid::Uuid;

use crate::global::{do_try, app_config};
use crate::global::prelude::*;

pub fn create_archive<F>(prefix: &str, func: F) -> Result
    where F: FnOnce(&str) -> Result {

    let app_config = app_config();

    let work_path = Path::new(&app_config.archive_config.temp_path)
        .combine_with(&Uuid::new_v4().to_string());

    do_try::run(|| {

        bash_exec!("mkdir -p {0} && chmod 777 {0}", &work_path.get_as_string()?);

        let uncompressed = work_path
            .combine_with("uncompressed-archive")
            .get_as_string()?;

        bash_exec!("mkdir -p {0} && chmod 777 {0}", uncompressed);

        func(&*uncompressed)?;

        let compressed = work_path
            .combine_with("compressed-archive.tar.gz")
            .get_as_string()?;

        bash_exec!(
            "cd {0} && tar -cf {1} --use-compress-program=pigz *",
            uncompressed,
            compressed
        );

        bash_exec!("rm {0} -rf", uncompressed);

        let final_archive = work_path
            .combine_with("final.enc")
            .get_as_string()?;

        bash_exec!(
            "openssl enc -aes-256-cbc -e -p -pass pass:{0} -in {1} -out {2}",
            &app_config.archive_config.archive_password,
            &compressed,
            &final_archive
        );

        bash_exec!("rm {0} -f", compressed);

        let now = app_start_time();

        let daily_folder = Path::new(&app_config.archive_config.cache_path)
            .combine_with(&now.format("day_%Y_%m_%d").to_string())
            .create_directory()?;

        let archive_file_name = format!(
            "{}.{}.{}.backup",
            prefix,
            now.format("%Y-%m-%d").to_string(),
            now.timestamp().to_string()
        );

        let archive_file_path = daily_folder
            .combine_with(&archive_file_name)
            .get_as_string()?;

        bash_exec!("mv {} {}", &final_archive, &archive_file_path);

        Ok(())
    }).finally(|| {

        bash_exec!( "rm {} -rf", work_path.get_as_string()?);

        Ok(())
    })?;

    Ok(())
}
