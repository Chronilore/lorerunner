use aws_config::SdkConfig;

pub struct AmazonWebServicesConfiguration {
    pub sdk_configuration: SdkConfig,
}

impl AmazonWebServicesConfiguration {
    pub async fn new() -> Self {
        AmazonWebServicesConfiguration {
            sdk_configuration: aws_config::load_from_env().await,
        }
    }
}
