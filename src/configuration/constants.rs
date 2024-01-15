pub mod project_file_paths {
    pub const CONFIGURATION_FILE_PATH: &str = "./private/configuration.ron";
    pub const FRONTEND_PATH: &str = "";
    pub const FRONTEND_PKG_PATH: &str = "";
    pub const GITHUB_PAGES_REPOSITORY_PATH: &str = "";
    pub const DIST_FOLDER: &str = "dist";
    pub const PKG_FOLDER: &str = "pkg";
}

pub mod command_line {
    pub const LIST_COMMAND: &str = "ls";
    pub const LIST_COMMAND_ALL_FLAG: &str = "-a";

    pub const PERSEUS_COMMAND: &str = "perseus";
    pub const PERSEUS_DEPLOY_COMMAND: &str = "deploy";
    pub const PERSEUS_DEPLOY_EXPORT_FLAG: &str = "-e";
}

pub mod http {
    pub const HTTP_HEADER_ACCEPT: &str = "Accept";
    pub const HTTP_HEADER_AUTHORIZATION: &str = "Authorization";
    pub mod github {
        pub const HTTP_HEADER_GITHUB_API_VERSION: &str = "X-GitHub-Api-Version";
    }
}

pub mod github {
    pub const MAX_GITHUB_JWT_EXPIRATION_MINUTES: u64 = 10;
    pub const GITHUB_API_URL: &str = "https://api.github.com";
    pub const GITHUB_API_VERSION: &str = "2022-11-28";
    pub const GET_GITHUB_APP_API_ROUTE: &str = "/app";
    pub const GITHUB_API_ACCEPT_HEADER_VALUE: &str = "application/vnd.github+json";
}

pub mod environment_variables {
    pub const AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
    pub const AWS_SECRET_ACCESS_KEY: &str = "AWS_SECRET_ACCESS_KEY";
    pub const AWS_REGION: &str = "AWS_REGION";
}

pub mod amazon_web_services {
    pub const RUST_ARM_NANO_TEMPLATE_NAME: &str = "rust-arm";
    pub const HTTPS_ALLOWED_SECURITY_GROUP_NAME: &str = "HTTPS Allowed";
    pub const PUBLIC_WEB_SERVER_SECURITY_GROUP_NAME: &str = "Public Web Server";
}

pub mod networking {
    pub const TCP_PROTOCOL: &str = "tcp";
    pub const HTTP_PORT: i32 = 80_i32;
    pub const HTTPS_PORT: i32 = 443_i32;
    pub const POSTGRES_PORT: i32 = 5432_i32;
    pub const ANYWHERE_IPV4: &str = "0.0.0.0/0";
    pub const ANYWHERE_IPV6: &str = "::/0";
}
