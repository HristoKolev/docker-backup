#[macro_use]
extern crate derive_more;

use crate::errors::GeneralError;
use crate::errors::GeneralError::{Dynamic, Detailed, IoError};

mod bash_shell;
mod errors;

fn main() {

    let res = main_result();

    match res {
       Ok(_) => {},
       Err(general_error) => {
           match general_error {
               IoError(io) => print!("{:#?}", io),
               Dynamic(dy) => print!("{:#?}", dy),
               Detailed(detail) => print!("{:#?}", detail),
           }
       }
    }
}

fn main_result () -> Result<(), GeneralError> {

    let res = bash_shell::exec("echo 123")?.as_result();

    println!("{:#?}", res.unwrap());

    Ok(())
}