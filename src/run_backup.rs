use crate::global::{bash_shell, do_try, app_config};
use crate::global::prelude::*;
use uuid::Uuid;
use std::path::Path;

pub fn run_backup() -> Result<()> {

    let app_config = app_config();

    let uuid = Uuid::new_v4().to_string();

    let work_path = Path::new(&app_config.archive_config.temp_path)
        .combine_with(&uuid)
        .get_as_string()?;

    do_try::run(|| {

        bash_exec!("mkdir -p {0} && chmod 777 {0}", &work_path);

        let uncompressed = Path::new(&work_path)
            .combine_with("uncompressed-archive")
            .get_as_string()?;

        bash_exec!("mkdir -p {0} && chmod 777 {0}", &uncompressed);

        // Do things.

        let compressed = Path::new(&work_path)
            .combine_with("compressed-archive.tar.gz")
            .get_as_string()?;

        bash_exec!(
            "cd {0} && tar -cf {1} --use-compress-program=pigz *",
            uncompressed,
            compressed
        );

        bash_exec!("rm {0} -rf", uncompressed);

        let final_archive = Path::new(&work_path)
            .combine_with("final.enc")
            .get_as_string()?;

        bash_exec!(
            "openssl enc -aes-256-cbc -e -p -pass pass:{0} -in {1} -out {2}",
            &app_config.archive_config.archive_password,
            &compressed,
            &final_archive
        );

        bash_exec!("rm {0} -f", compressed);

        Ok(())
    }).finally(|| {

        bash_exec!( "rm {} -rf", work_path);

        Ok(())
    })?;

    Ok(())
}
