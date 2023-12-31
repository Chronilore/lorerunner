use anyhow::Result;
use jwt_simple::prelude::{Claims, Duration, JWTClaims, RS256KeyPair, RSAKeyPairLike};
use log::{error, info};

use crate::{
    configuration::constants::{
        github::{
            GET_GITHUB_APP_API_ROUTE, GITHUB_API_ACCEPT_HEADER_VALUE, GITHUB_API_URL,
            GITHUB_API_VERSION, MAX_GITHUB_JWT_EXPIRATION_MINUTES,
        },
        http::{
            github::HTTP_HEADER_GITHUB_API_VERSION, HTTP_HEADER_ACCEPT, HTTP_HEADER_AUTHORIZATION,
        },
    },
    file_system::get_file_content_as_string,
    http::format_authorization_bearer_token,
};

pub struct GitHubApp {
    json_web_token: Option<String>,
}

impl GitHubApp {
    pub fn new(private_key_path: &str, github_app_id: u32) -> Result<Self> {
        let json_web_token: String =
            create_json_web_token(private_key_path, &github_app_id.to_string())?;
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
            .set(
                HTTP_HEADER_AUTHORIZATION,
                &format_authorization_bearer_token(token),
            )
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
        info!("[GET][{}{}]", GITHUB_API_URL, GET_GITHUB_APP_API_ROUTE);
        match http_agent
            .get(format!("{}{}", GITHUB_API_URL, GET_GITHUB_APP_API_ROUTE).as_str())
            .set(HTTP_HEADER_ACCEPT, GITHUB_API_ACCEPT_HEADER_VALUE)
            .set(HTTP_HEADER_GITHUB_API_VERSION, GITHUB_API_VERSION)
            .set(
                HTTP_HEADER_AUTHORIZATION,
                &format_authorization_bearer_token(token),
            )
            .call()
        {
            Ok(reply) => {
                info!(
                    "\n[GET][{}{}][Status Code: {}][Status Text: {}]",
                    GITHUB_API_URL,
                    GET_GITHUB_APP_API_ROUTE,
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
    let key_pair: RS256KeyPair = RS256KeyPair::from_pem(&private_key_string)?;
    let claims: JWTClaims<jwt_simple::prelude::NoCustomClaims> =
        Claims::create(Duration::from_mins(MAX_GITHUB_JWT_EXPIRATION_MINUTES)).with_issuer(issuer);
    let token: String = key_pair.sign(claims)?;

    Ok(token)
}
