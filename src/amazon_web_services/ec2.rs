use anyhow::Result;
use aws_sdk_ec2::operation::{
    describe_launch_templates::{
        builders::DescribeLaunchTemplatesFluentBuilder, DescribeLaunchTemplatesOutput,
    },
    describe_security_groups::{
        builders::DescribeSecurityGroupsFluentBuilder, DescribeSecurityGroupsOutput,
    },
    describe_vpcs::DescribeVpcsOutput,
};
use log::debug;
pub const RUST_ARM_NANO_TEMPLATE_NAME: &str = "rust-arm";

pub enum AwsQueryType {
    Id,
    Name,
    None,
}

pub async fn create_ec2_launch_template_version(client: aws_sdk_ec2::Client) -> Result<()> {
    client.create_launch_template_version().send().await?;

    Ok(())
}

pub async fn get_ec2_launch_templates(
    client: &aws_sdk_ec2::Client,
    query_type: AwsQueryType,
    query: Vec<String>,
) -> Result<DescribeLaunchTemplatesOutput> {
    let mut query_builder: DescribeLaunchTemplatesFluentBuilder =
        client.describe_launch_templates();

    match query_type {
        AwsQueryType::Id => query_builder = query_builder.set_launch_template_names(Some(query)),
        AwsQueryType::Name => query_builder = query_builder.set_launch_template_names(Some(query)),
        AwsQueryType::None => (),
    }

    let result: DescribeLaunchTemplatesOutput = query_builder.send().await?;

    result
        .launch_templates()
        .iter()
        .for_each(|template| debug!("Launch Template: {:?}", template));

    Ok(result)
}

pub async fn get_vpcs(
    client: &aws_sdk_ec2::Client,
    vpc_ids_filter: Option<Vec<String>>,
) -> Result<DescribeVpcsOutput> {
    let result: DescribeVpcsOutput = client
        .describe_vpcs()
        .set_vpc_ids(vpc_ids_filter)
        .send()
        .await?;

    result
        .vpcs()
        .iter()
        .for_each(|vpc| debug!("VPC: {:?}", vpc));

    Ok(result)
}

pub async fn get_security_groups(
    client: &aws_sdk_ec2::Client,
    query_type: AwsQueryType,
    query: Vec<String>,
) -> Result<DescribeSecurityGroupsOutput> {
    let mut query_builder: DescribeSecurityGroupsFluentBuilder = client.describe_security_groups();

    match query_type {
        AwsQueryType::Id => query_builder = query_builder.set_group_ids(Some(query)),
        AwsQueryType::Name => query_builder = query_builder.set_group_names(Some(query)),
        AwsQueryType::None => (),
    }

    let result: DescribeSecurityGroupsOutput = query_builder.send().await?;

    result
        .security_groups()
        .iter()
        .for_each(|security_group| debug!("Security Group: {:?}", security_group));

    Ok(result)
}
