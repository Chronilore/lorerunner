use anyhow::Result;

use aws_config::SdkConfig;
use aws_sdk_ec2::Client;
use log::info;

use crate::amazon_web_services::ec2;
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

    println!();
    let aws_configuration: SdkConfig = aws_config::load_from_env().await;

    let ec2_client = Client::new(&aws_configuration);
    ec2::get_ec2_launch_templates(ec2_client).await?;

    Ok(())
}
