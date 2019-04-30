use serde::{Serialize, Deserialize};
use crate::errors::GeneralError;

#[derive(Serialize, Deserialize)]
pub struct EmailConfig {
    pub email_enabled: bool,
    pub notification_emails: Vec<String>,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct DockerConfig {
    pub volumes_path: String,
    pub volumes_mirror_path: String,
    pub archive_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub hostname: String,
    pub email_config: EmailConfig,
    pub docker_config: DockerConfig,
}

pub fn read_config() -> Result<AppConfig, GeneralError> {

    let app_config_path = "/work/projects/docker-backup/app-config.json";

    let json_content = std::fs::read_to_string(app_config_path)?;

    let materialized = serde_json::from_str(&*json_content)?;

    Ok(materialized)
}
