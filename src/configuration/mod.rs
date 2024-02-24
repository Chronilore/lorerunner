use anyhow::Result;
use env_logger::{Builder, Target};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::io::Write;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::file_system::{create_file_if_missing, get_file_content_as_string, CreateFileResult};

use self::constants::project_file_paths::CONFIGURATION_FILE_PATH;

pub mod constants;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub log_level: LevelFilter,
    pub github_app_private_key_path: String,
    pub github_app_id: u32,
    pub github_app_installation_id: u32,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            log_level: LevelFilter::Debug,
            github_app_private_key_path: String::new(),
            github_app_id: 0_u32,
            github_app_installation_id: 0_u32,
        }
    }
}

pub fn get_application_configuration() -> Result<Configuration> {
    if let CreateFileResult::FileCreated =
        create_file_if_missing(CONFIGURATION_FILE_PATH.to_string())?
    {
        let file_content: String = ron::to_string(&Configuration::new()).expect(
            "Failed to create default application configuration file when one was not found",
        );
        std::fs::write(CONFIGURATION_FILE_PATH, file_content)?;
        panic!("Application configuration file not found: {}\nCreating new file with default configuration.", CONFIGURATION_FILE_PATH);
    }
    let configuration_file_content: String =
        get_file_content_as_string(CONFIGURATION_FILE_PATH.to_string())?;
    let configuration: Configuration = ron::from_str(&configuration_file_content).expect(
        format!(
            "Failed to parse ron configuration from file: {}",
            CONFIGURATION_FILE_PATH
        )
        .as_str(),
    );
    Ok(configuration)
}

pub fn configure_logging(configuration: &Configuration) -> Result<()> {
    Builder::new()
        .target(Target::Stdout)
        .format(move |buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "[LORERUNNER]: [{}] [{}] - {}",
                OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                record.level(),
                record.args()
            )
        })
        .filter(None, configuration.log_level)
        .init();

    Ok(())
}
