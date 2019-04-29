#[macro_use]
extern crate derive_more;

extern crate lettre;
extern crate lettre_email;
extern crate failure;

extern crate serde;
extern crate serde_json;

use crate::errors::{GeneralError, handle_error, DetailedError};
use crate::app_config::AppConfig;

mod bash_shell;
mod errors;
mod do_try;
mod email;
mod app_config;

fn main() {

    let result = main_result();

    match result {
        Ok(..) => {},
        Err(error) => {
            handle_error(error)
        }
    }
}

pub fn main_result () -> Result<(), GeneralError> {

    let app_config = app_config::read_config()?;

    let res = run_backup(&app_config);

    match res {
        Ok(..) => {},
        Err(err) => {

            let subject = format!(
                "An error occurred while running `docker-backup` on `{}`.",
                app_config.hostname
            );

            let content = format!("{}", 1);

            send_mail(
                &app_config,
                &*subject,
                &*subject,
            )?;
        }
    }

    Ok(())
}

pub fn run_backup(app_config: &AppConfig) -> Result<(), GeneralError> {

    Ok(())
}

fn send_mail(app_config: &AppConfig, subject: &str, content: &str) -> Result<(), GeneralError> {

    let email_client = email::EmailClient::new(
        &*app_config.email_config.smtp_username,
        &*app_config.email_config.smtp_password,
        &*app_config.email_config.smtp_host,
        app_config.email_config.smtp_port,
    );

    let message_subject = "subj123";
    let message_content = "content123";

    let message = email::EmailMessage::new(
        app_config.email_config.notification_emails.clone(),
        message_subject,
        message_content,
    );

    email_client.send(message)?;

    Ok(())
}
