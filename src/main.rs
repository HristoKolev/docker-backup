extern crate lettre;
extern crate lettre_email;
extern crate failure;

extern crate serde;
extern crate serde_json;

mod bash_shell;
mod errors;
mod do_try;
mod email;
mod app_config;
mod email_report;
mod run_backup;

use crate::errors::*;
use crate::run_backup::run_backup;

fn main() {

    std::env::set_var("RUST_BACKTRACE", "1");

    if let Err(error) = main_result() {

        handle_error(&error)
            .expect("An error occurred while handling an error.");
    }
}

pub fn main_result () -> Result<()> {

    let app_config = app_config::read_config()?;

    let result = run_backup(&app_config);

    match result {
        Ok(..) => {},
        Err(err) => {
            email_report::send_report(&app_config, &err)?;
            println!("{:#?}", err);
        }
    }

    Ok(())
}
