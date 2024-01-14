use anyhow::Result;

use aws_config::SdkConfig;
use aws_sdk_ec2::Client;
use log::info;

use crate::amazon_web_services::ec2::{self, RUST_ARM_NANO_TEMPLATE_NAME};
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
    let configuration: Configuration = get_application_configuration()?;
    configuration::configure_logging(&configuration)?;
    info!("lorerunner started");

    // test_github(&configuration).await?;

    println!();
    let aws_configuration: SdkConfig = aws_config::load_from_env().await;

    let ec2_client: Client = Client::new(&aws_configuration);
    ec2::get_ec2_launch_templates(
        &ec2_client,
        ec2::AwsQueryType::Name,
        [RUST_ARM_NANO_TEMPLATE_NAME.to_string()].to_vec(),
    )
    .await?;
    ec2::get_vpcs(&ec2_client, None).await?;
    ec2::get_security_groups(&ec2_client, ec2::AwsQueryType::None, Vec::new()).await?;

    Ok(())
}

pub async fn test_github(configuration: &Configuration) -> Result<()> {
    let github_app: GitHubApp = GitHubApp::new(
        &configuration.github_app_private_key_path,
        configuration.github_app_id,
    )?;

    github_app.ping_github()?;
    github_app.get_app_details()?;

    Ok(())
}
