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
