use serde::{Serialize, Deserialize};

use super::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailConfig {
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
    pub min_archive_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArchiveConfig {
    pub cache_path: String,
    pub temp_path: String,
    pub gpg_key_name: String,
    pub cache_expiry_days: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerConfig {
    pub volumes_path: String,
    pub archive_config: Option<ArchiveConfig>,
    pub remote_config: Option<Vec<RemoteConfig>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub hostname: String,
    pub sentry_dsn: String,
    pub email_config: EmailConfig,
    pub archive_config: ArchiveConfig,
    pub docker_config: Option<HashMap<String, DockerConfig>>,
    pub remote_config: Option<Vec<RemoteConfig>>,
}

pub fn read_config(file_path: &str) -> Result<AppConfig> {
    let json_content = ::std::fs::read_to_string(file_path)?;
    let materialized = serde_json::from_str(&json_content)?;
    Ok(materialized)
}
