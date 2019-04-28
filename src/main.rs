#[macro_use]
extern crate derive_more;

use crate::errors::{GeneralError, handle_error};

mod bash_shell;
mod errors;
mod do_try;

fn main() {

    let result = main_result();

    if let Err(error) = result {
        handle_error(error)
    }
}

pub fn main_result () -> Result<(), GeneralError> {

    let volumes_path = "/var/lib/docker/volumes";
    let volumes_copy_path = format!("{}-copy", volumes_path);
    let volumes_archive_path = "/var/lib/docker/volumes.tar.gz";

    let ps_result = bash_shell::exec("echo `docker ps -a -q`")?.as_result()?;

    do_try::run(|| {

        do_try::run(|| {

            bash_shell::exec(&format!("rsync -a {}/ {}/", volumes_path, volumes_copy_path))?.as_result()?;

            bash_shell::exec(&format!("docker pause {}", ps_result.stdout))?.as_result()?;

            bash_shell::exec(&format!("rsync -a {}/ {}/", volumes_path, volumes_copy_path))?.as_result()?;

            Ok(())

        }).finally(|| {

            bash_shell::exec(&format!("docker unpause {}", ps_result.stdout))?.as_result()?;

            Ok(())
        })?;

        bash_shell::exec(&format!(
            "cd {} && tar -cpf {} --use-compress-program=\"pigz\" ./",
            volumes_copy_path,
            volumes_archive_path
        ))?.as_result()?;

        Ok(())
        
    }).finally(|| {

        bash_shell::exec(&format!("rm {} -rf", volumes_copy_path))?.as_result()?;

        Ok(())
    })?;

   Ok(())
}