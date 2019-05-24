use serde::{Serialize, Deserialize};

use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailConfig {
    pub email_enabled: bool,
    pub notification_emails: Vec<String>,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerConfig {
    pub volumes_path: String,
    pub volumes_mirror_path: String,
    pub archive_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub hostname: String,
    pub sentry_dsn: String,
    pub email_config: EmailConfig,
    pub docker_config: DockerConfig,
}

pub fn read_config(file_path: &str) -> Result<AppConfig> {
    let json_content = std::fs::read_to_string(file_path)?;
    let materialized = serde_json::from_str(&*json_content)?;
    Ok(materialized)
}
