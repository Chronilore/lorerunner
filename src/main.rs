use anyhow::Result;

use aws_config::SdkConfig;
use aws_sdk_ec2::Client;
use log::info;

use crate::amazon_web_services::ec2::{
    self, add_security_group_ingress, remove_security_group_ingress,
};
use crate::configuration::constants::amazon_web_services::{
    PUBLIC_WEB_SERVER_SECURITY_GROUP_NAME, RUST_ARM_NANO_TEMPLATE_NAME,
};
use crate::configuration::constants::networking::{ANYWHERE_IPV4, HTTPS_PORT, TCP_PROTOCOL};
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
    test_aws().await?;

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

pub async fn test_aws() -> Result<()> {
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
    let security_group_result = ec2::get_security_groups(
        &ec2_client,
        ec2::AwsQueryType::Name,
        vec![PUBLIC_WEB_SERVER_SECURITY_GROUP_NAME.to_string()],
    )
    .await?;

    if let Some(mut security_group) = security_group_result.security_groups {
        if let Some(group) = security_group.first_mut() {
            add_security_group_ingress(
                &ec2_client,
                group.group_id.clone(),
                Some(ANYWHERE_IPV4.to_string()),
                Some(TCP_PROTOCOL.to_string()),
                Some(HTTPS_PORT),
                Some(HTTPS_PORT),
            )
            .await?;
            remove_security_group_ingress(
                &ec2_client,
                group.group_id.clone(),
                Some(ANYWHERE_IPV4.to_string()),
                Some(TCP_PROTOCOL.to_string()),
                Some(HTTPS_PORT),
                Some(HTTPS_PORT),
            )
            .await?;
        };
    };
    Ok(())
}
