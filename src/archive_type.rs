use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::global::prelude::*;
use crate::docker_volumes::create_docker_volumes_archive;

#[derive(Debug, EnumIter)]
pub enum ArchiveType {
    DockerVolumes
}

pub fn parse_archive_type(prefix: &str) -> Result<ArchiveType> {
    match prefix {
        "docker-volumes" => Ok(ArchiveType::DockerVolumes),
        _ => Err(CustomError::user_error(&format!("Archive type not found: {}", prefix)))
    }
}

pub fn get_archive_config(archive_type: ArchiveType) -> ArchiveConfig {

    let app_config = app_config();

    let archive_config = match archive_type {
        ArchiveType::DockerVolumes => app_config.docker_config.clone()
            .map(|x| x.archive_config)
            .flatten()
    };

    archive_config.unwrap_or(app_config.archive_config.clone())
}

pub fn get_remote_config(archive_type: ArchiveType) -> Vec<RemoteConfig> {

    let app_config = app_config();

    let custom_config = match archive_type {
        ArchiveType::DockerVolumes => app_config.docker_config.clone()
            .map(|x| x.remote_config)
            .flatten()
    };

    match custom_config {
        Some(x) => x,
        None => match app_config.remote_config.clone() {
            Some(x) => x,
            None => Vec::new()
        }
    }
}

impl ArchiveType {

    pub fn all() -> Vec<ArchiveType> {
        ArchiveType::iter().collect::<Vec<ArchiveType>>()
    }
}

pub fn get_create_archive(archive_type: &ArchiveType) -> impl FnOnce(&str) -> Result {

    match archive_type {
        ArchiveType::DockerVolumes => create_docker_volumes_archive
    }
}