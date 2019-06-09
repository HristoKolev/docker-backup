use crate::global::prelude::*;

pub trait ArchiveConfigExtensions {

    fn as_config(&self) -> ArchiveConfig;
}

impl ArchiveConfigExtensions for CustomArchiveConfig {

    fn as_config(&self) -> ArchiveConfig {

        let archive_config = &app_config().archive_config;

        ArchiveConfig {
            temp_path: self.temp_path.clone().unwrap_or_else(|| archive_config.temp_path.clone()),
            cache_path: self.cache_path.clone().unwrap_or_else(|| archive_config.cache_path.clone()),
            archive_password: self.archive_password.clone().unwrap_or_else(|| archive_config.archive_password.clone()),
            cache_expiry_days: self.cache_expiry_days.clone().unwrap_or_else(|| archive_config.cache_expiry_days.clone()),
        }
    }
}

impl ArchiveConfigExtensions for Option<CustomArchiveConfig> {
    fn as_config(&self) -> ArchiveConfig {

        let archive_config = &app_config().archive_config;

        ArchiveConfig {
            temp_path: self.map(|x| x.temp_path.clone()).flatten().unwrap_or_else(|| archive_config.temp_path.clone()),
            cache_path: self.map(|x| x.cache_path.clone()).flatten().unwrap_or_else(|| archive_config.cache_path.clone()),
            archive_password: self.map(|x| x.archive_password.clone()).flatten().unwrap_or_else(|| archive_config.archive_password.clone()),
            cache_expiry_days: self.map(|x| x.cache_expiry_days.clone()).flatten().unwrap_or_else(|| archive_config.cache_expiry_days.clone()),
        }
    }
}