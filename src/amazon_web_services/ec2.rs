pub const RUST_ARM_NANO_TEMPLATE_NAME: &str = "rust-arm-nano";

pub async fn create_ec2_launch_template(client: aws_sdk_ec2::Client) {
    let launch_template = client.create_launch_template_version();
}
