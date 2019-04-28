#[macro_use]
extern crate derive_more;

use crate::errors::{GeneralError, handle_error};

mod bash_shell;
mod errors;

fn main() {

    let result = main_result();

    if let Err(error) = result {
        handle_error(error)
    }
}

fn main_result () -> Result<(), GeneralError> {

    let result = bash_shell::exec("docker ps")?.as_result()?;

    println!("{:#?}", result);

    Ok(())
}