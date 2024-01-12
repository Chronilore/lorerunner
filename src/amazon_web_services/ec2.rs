use anyhow::Result;
use aws_sdk_ec2::operation::describe_launch_templates::DescribeLaunchTemplatesOutput;
pub const RUST_ARM_NANO_TEMPLATE_NAME: &str = "rust-arm";

pub async fn create_ec2_launch_template(client: aws_sdk_ec2::Client) {
    let launch_template = client.create_launch_template_version();
}

pub async fn get_ec2_launch_templates(client: aws_sdk_ec2::Client) -> Result<()> {
    let result: DescribeLaunchTemplatesOutput = client
        .describe_launch_templates()
        .set_launch_template_names(Some([RUST_ARM_NANO_TEMPLATE_NAME.to_string()].to_vec()))
        .send()
        .await?;

    result.launch_templates().iter().for_each(|template| {
        println!(
            "Launch Template:\nid: {:<12} name: {:<10} latest_version_number: {:<4}",
            template
                .launch_template_id
                .as_ref()
                .map_or_else(|| "?".to_string(), |id| id.to_string()),
            template
                .launch_template_name
                .as_ref()
                .map_or_else(|| "?".to_string(), |name| name.to_string()),
            template
                .latest_version_number
                .as_ref()
                .map_or_else(|| &-1i64, |number| number)
        )
    });

    Ok(())
}
