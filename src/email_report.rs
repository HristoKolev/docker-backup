use crate::app_config::AppConfig;
use crate::errors::GeneralError;
use crate::email;

pub fn send_report(app_config: &AppConfig, error: &GeneralError) -> Result<(), GeneralError> {

    let subject = format!(
        "An error occurred while running `docker-backup` on `{}`.",
        app_config.hostname
    );

    let content = format!("{:#?}", error);

    send_mail(
        &app_config,
        &*subject,
        &*content,
    )?;

    Ok(())
}

fn send_mail(app_config: &AppConfig, subject: &str, content: &str) -> Result<(), GeneralError> {

    let email_client = email::EmailClient::new(
        &*app_config.email_config.smtp_username,
        &*app_config.email_config.smtp_password,
        &*app_config.email_config.smtp_host,
        app_config.email_config.smtp_port,
    );

    let message = email::EmailMessage::new(
        app_config.email_config.notification_emails.clone(),
        subject,
        content,
    );

    email_client.send(message)?;

    Ok(())
}
