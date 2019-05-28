use serde_json::json;
use handlebars::Handlebars;

use super::email;
use super::app_config;
use super::prelude::*;
use crate::global::logger;

fn render_report(error: &CustomError) -> Result<String> {

    let html_template = include_str!("email-template.html");

    let app_config = app_config();

    let logs = logger().get_logs()?.join("\n");

    let registry = Handlebars::new();

    let rendered = registry.render_template(
        html_template,
        &json!({
            "app_config": app_config,
            "formatted_error": format!("{:#?}", error),
            "logs": logs,
         })
    )?;

    Ok(rendered)
}

pub fn send_report(error: &CustomError) -> Result {

    let app_config = app_config();

    let subject = format!(
        "[FAILURE] An error occurred while running `docker-backup` on `{}`.",
        app_config.hostname
    );

    let report_content = render_report(&error)?;

    send_mail(&*subject, &*report_content)?;

    Ok(())
}

fn send_mail(subject: &str, content: &str) -> Result {

    let app_config = app_config();

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

    email_client.send(&message)?;

    Ok(())
}
