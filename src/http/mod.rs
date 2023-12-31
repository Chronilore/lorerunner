pub fn format_authorization_bearer_token(token: &str) -> String {
    format!("Bearer {}", token)
}
