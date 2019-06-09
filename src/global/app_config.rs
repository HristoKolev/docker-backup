use serde::{Serialize, Deserialize};

use super::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailConfig {
    pub email_enabled: bool,
    pub notification_emails: Vec<String>,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteConfig {
    pub remote_name: String,
    pub remote_path: String,
    pub cache_expiry_days: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomRemoteConfig {
    pub remote_name: Option<String>,
    pub remote_path: Option<String>,
    pub cache_expiry_days: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArchiveConfig {
    pub cache_path: String,
    pub temp_path: String,
    pub archive_password: String,
    pub cache_expiry_days: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomArchiveConfig {
    pub cache_path: Option<String>,
    pub temp_path: Option<String>,
    pub archive_password: Option<String>,
    pub cache_expiry_days: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerConfig {
    pub volumes_path: String,
    pub custom_archive_config: Option<CustomArchiveConfig>,
    pub custom_remote_config: Option<Vec<CustomRemoteConfig>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub hostname: String,
    pub sentry_dsn: String,
    pub email_config: EmailConfig,
    pub archive_config: ArchiveConfig,
    pub docker_config: Option<DockerConfig>,
    pub remote_config: Option<Vec<RemoteConfig>>,
}

pub fn read_config(file_path: &str) -> Result<AppConfig> {
    let json_content = ::std::fs::read_to_string(file_path)?;
    let materialized = serde_json::from_str(&json_content)?;
    Ok(materialized)
}
