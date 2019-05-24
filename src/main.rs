#![forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod global;

mod run_backup;

use crate::global::prelude::*;
use crate::run_backup::run_backup;

fn main() {

    global::initialize();

    main_result().crash_on_error();
}

fn main_result() -> Result<()> {

    // run_backup()?;


    return Err(CustomError::from_message("cats"));

    Ok(())
}
