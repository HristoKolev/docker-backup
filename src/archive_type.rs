use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::global::prelude::*;
use crate::archive_config_extensions::*;
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
            .map(|x| x.custom_archive_config)
            .flatten()
    };

    archive_config.as_config()
}

impl ArchiveType {

    pub fn all() -> Vec<ArchiveType> {
        ArchiveType::iter().collect::<Vec<ArchiveType>>()
    }
}

pub fn get_create_archive(archive_type: &ArchiveType) -> impl FnOnce(&str) -> Result {

    let s = match archive_type {
        ArchiveType::DockerVolumes => create_docker_volumes_archive
    };

    s
}