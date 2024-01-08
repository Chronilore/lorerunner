pub type AwsIamPolicyName = &'static str;
pub type AwsIamPolicyDocument = &'static str;

pub async fn create_policy(
    client: aws_sdk_iam::Client,
    policy_name: AwsIamPolicyName,
    policy_document: AwsIamPolicyDocument,
) -> Result<aws_sdk_iam::types::Policy, aws_sdk_iam::Error> {
    let policy = client
        .create_policy()
        .policy_name(policy_name)
        .policy_document(policy_document)
        .send()
        .await?;
    Ok(policy.policy.unwrap())
}
