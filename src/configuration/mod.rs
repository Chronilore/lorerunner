use anyhow::Result;
use env_logger::{Builder, Target};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::io::Write;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::file_system::get_file_content_as_string;

use self::constants::project_file_paths::CONFIGURATION_FILE_PATH;

pub mod constants;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub github_app_private_key_path: String,
    pub github_app_id: u32,
    pub github_app_installation_id: u32,
}

pub fn get_application_configuration() -> Result<Configuration> {
    let configuration_file_content: String =
        get_file_content_as_string(CONFIGURATION_FILE_PATH.to_string())?;
    let configuration: Configuration = ron::from_str(&configuration_file_content)?;
    Ok(configuration)
}

pub fn configure_logging() -> Result<()> {
    const DEBUG: &str = "DEBUG";
    Builder::new()
        .target(Target::Stdout)
        .format(move |buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "[LORERUNNER_{}]: [{}] [{}] - {}",
                DEBUG,
                OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    Ok(())
}
