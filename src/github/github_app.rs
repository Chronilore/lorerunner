use std::borrow::Borrow;

use anyhow::Result;
use jwt_simple::prelude::{Claims, Duration, JWTClaims, RS256KeyPair, RSAKeyPairLike};
use log::{error, info};

use crate::get_file_content_as_string;

const PRIVATE_KEY_PATH: &str = "";
const APP_ID: u32 = 401200;
const MAX_GITHUB_EXPIRATION_MINUTES: u64 = 10;
const TEST_REPOSITORY_INSTALLATION_ID: u32 = 1;
const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_API_VERSION: &str = "2022-11-28";
const GITHUB_API_ACCEPT_HEADER_VALUE: &str = "application/vnd.github+json";
const GET_GITHUB_APP_API_ROUTE: &str = "/app";

const HTTP_HEADER_ACCEPT: &str = "Accept";
const HTTP_HEADER_GITHUB_API_VERSION: &str = "X-GitHub-Api-Version";
const HTTP_HEADER_AUTHORIZATION: &str = "Authorization";

pub struct GitHubApp {
    json_web_token: Option<String>,
}

impl GitHubApp {
    pub fn new() -> Result<Self> {
        let json_web_token: String = create_json_web_token(PRIVATE_KEY_PATH, &APP_ID.to_string())?;
        Ok(GitHubApp {
            json_web_token: Some(json_web_token),
        })
    }

    fn get_http_agent() -> ureq::Agent {
        ureq::Agent::new()
    }

    pub fn ping_github(&self) -> Result<()> {
        info!("Pinging GitHub");
        let http_agent: ureq::Agent = Self::get_http_agent();
        let Some(token) = &self.json_web_token else {
            panic!("Expected a auth token but there wasn't one!");
        };
        match http_agent
            .get(GITHUB_API_URL)
            .set(HTTP_HEADER_ACCEPT, GITHUB_API_ACCEPT_HEADER_VALUE)
            .set(HTTP_HEADER_GITHUB_API_VERSION, GITHUB_API_VERSION)
            .set(HTTP_HEADER_AUTHORIZATION, &format!("Bearer {}", token))
            .call()
        {
            Ok(reply) => {
                info!(
                    "[GET][{}] Status Code: {} Status Text{}",
                    GITHUB_API_URL,
                    reply.status(),
                    reply.status_text()
                );
                info!("\nBody: {}", reply.into_string()?);
            }
            Err(ureq::Error::Status(code, response)) => {
                error!(
                    "[GET][{}] Status Code: {} Status Text: {} - Response: {}",
                    GITHUB_API_URL,
                    code,
                    response.status_text().to_string(),
                    response.into_string()?
                );
            }
            Err(error) => {
                error!("{}", error.to_string());
            }
        };

        Ok(())
    }

    pub fn get_app_details(&self) -> Result<()> {
        let http_agent: ureq::Agent = Self::get_http_agent();
        let Some(token) = &self.json_web_token else {
            panic!("Expected a auth token but there wasn't one!");
        };
        info!("Getting app details");
        match http_agent
            .get(format!("{}{}", GITHUB_API_URL, GET_GITHUB_APP_API_ROUTE).as_str())
            .set(HTTP_HEADER_ACCEPT, GITHUB_API_ACCEPT_HEADER_VALUE)
            .set(HTTP_HEADER_GITHUB_API_VERSION, GITHUB_API_VERSION)
            .set(HTTP_HEADER_AUTHORIZATION, &format!("Bearer {}", token))
            .call()
        {
            Ok(reply) => {
                info!(
                    "\nStatus Code: {} Status Text: {}",
                    reply.status(),
                    reply.status_text()
                );
                info!("\nBody: {}", reply.into_string()?);
            }
            Err(ureq::Error::Status(status_code, response)) => {
                error!(
                    "\nStatus Code: {} \nBody: {}\n",
                    status_code,
                    response.into_string()?
                );
            }
            Err(error) => {
                error!("{}", error.to_string());
            }
        };

        Ok(())
    }
}

pub fn create_json_web_token(key_path: &str, issuer: &str) -> Result<String> {
    let private_key_string: String = get_file_content_as_string(String::from(key_path))?;
    info!("\n{}\n", private_key_string);
    let key_pair: RS256KeyPair = RS256KeyPair::from_pem(&private_key_string)?;
    let claims: JWTClaims<jwt_simple::prelude::NoCustomClaims> =
        Claims::create(Duration::from_mins(MAX_GITHUB_EXPIRATION_MINUTES)).with_issuer(issuer);
    let token: String = key_pair.sign(claims)?;

    Ok(token)
}
