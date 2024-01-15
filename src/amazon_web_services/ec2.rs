use anyhow::Result;
use aws_sdk_ec2::{
    operation::{
        authorize_security_group_ingress::AuthorizeSecurityGroupIngressOutput,
        create_security_group::CreateSecurityGroupOutput,
        describe_launch_templates::{
            builders::DescribeLaunchTemplatesFluentBuilder, DescribeLaunchTemplatesOutput,
        },
        describe_security_groups::{
            builders::DescribeSecurityGroupsFluentBuilder, DescribeSecurityGroupsOutput,
        },
        describe_vpcs::DescribeVpcsOutput,
        modify_security_group_rules::ModifySecurityGroupRulesOutput,
        revoke_security_group_ingress::RevokeSecurityGroupIngressOutput,
    },
    types::{SecurityGroupRuleRequest, SecurityGroupRuleUpdate},
};
use log::debug;

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

pub async fn create_security_group(
    client: &aws_sdk_ec2::Client,
    name: Option<String>,
    description: Option<String>,
    vpc_id: Option<String>,
) -> Result<CreateSecurityGroupOutput> {
    let mut query_builder = client.create_security_group();

    query_builder = query_builder
        .set_description(name)
        .set_group_name(description)
        .set_vpc_id(vpc_id);

    let result: CreateSecurityGroupOutput = query_builder.send().await?;

    Ok(result)
}

pub async fn update_security_group(
    client: &aws_sdk_ec2::Client,
    security_group_id: Option<String>,
) -> Result<ModifySecurityGroupRulesOutput> {
    let security_group_rules = SecurityGroupRuleUpdate::builder()
        .set_security_group_rule(Some(SecurityGroupRuleRequest::builder().build()))
        .build();

    let query_builder = client
        .modify_security_group_rules()
        .set_group_id(security_group_id)
        .security_group_rules(security_group_rules);

    let result: ModifySecurityGroupRulesOutput = query_builder.send().await?;

    Ok(result)
}

pub async fn add_security_group_ingress(
    client: &aws_sdk_ec2::Client,
    security_group_id: Option<String>,
    cidr_ip: Option<String>,
    ip_protocol: Option<String>,
    port_range_start: Option<i32>,
    port_range_end: Option<i32>,
) -> Result<AuthorizeSecurityGroupIngressOutput> {
    let query_builder = client
        .authorize_security_group_ingress()
        .set_group_id(security_group_id)
        .set_cidr_ip(cidr_ip)
        .set_from_port(port_range_start)
        .set_to_port(port_range_end)
        .set_ip_protocol(ip_protocol);

    let result: AuthorizeSecurityGroupIngressOutput = query_builder.send().await?;

    Ok(result)
}

pub async fn remove_security_group_ingress(
    client: &aws_sdk_ec2::Client,
    security_group_id: Option<String>,
    cidr_ip: Option<String>,
    ip_protocol: Option<String>,
    port_range_start: Option<i32>,
    port_range_end: Option<i32>,
) -> Result<RevokeSecurityGroupIngressOutput> {
    let query_builder = client
        .revoke_security_group_ingress()
        .set_group_id(security_group_id)
        .set_cidr_ip(cidr_ip)
        .set_from_port(port_range_start)
        .set_to_port(port_range_end)
        .set_ip_protocol(ip_protocol);

    let result: RevokeSecurityGroupIngressOutput = query_builder.send().await?;

    Ok(result)
}

pub enum IpVersion {
    Ipv4,
    Ipv6,
}

pub async fn create_security_group_rule(
    ip_version: IpVersion,
    cidr_ip: Option<String>,
    ip_protocol: Option<String>,
    port_range_start: Option<i32>,
    port_range_end: Option<i32>,
    description: Option<String>,
) -> Result<SecurityGroupRuleUpdate> {
    let mut builder = SecurityGroupRuleRequest::builder();

    match ip_version {
        IpVersion::Ipv4 => builder = builder.set_cidr_ipv4(cidr_ip),
        IpVersion::Ipv6 => builder = builder.set_cidr_ipv6(cidr_ip),
    }

    builder = builder
        .set_description(description)
        .set_ip_protocol(ip_protocol)
        .set_from_port(port_range_start)
        .set_to_port(port_range_end);

    let rule = Some(builder.build());

    let security_group_rules: SecurityGroupRuleUpdate = SecurityGroupRuleUpdate::builder()
        .set_security_group_rule(rule)
        .build();

    Ok(security_group_rules)
}
