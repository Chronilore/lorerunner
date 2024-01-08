use amazon_web_services::configuration::AmazonWebServicesConfiguration;
use anyhow::Result;

use log::info;

use crate::configuration::{get_application_configuration, Configuration};
use crate::github::github_app::GitHubApp;

pub mod amazon_web_services;
pub mod configuration;
pub mod file_system;
pub mod github;
pub mod http;
pub mod workshop;

#[tokio::main]
async fn main() -> Result<()> {
    configuration::configure_logging()?;
    info!("lorerunner started");

    let configuration: Configuration = get_application_configuration()?;

    let github_app: GitHubApp = GitHubApp::new(
        &configuration.github_app_private_key_path,
        configuration.github_app_id,
    )?;

    github_app.ping_github()?;
    github_app.get_app_details()?;

    let aws_configuration: AmazonWebServicesConfiguration =
        AmazonWebServicesConfiguration::new().await;

    Ok(())
}
