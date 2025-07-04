use gcp_auth::{provider, Token};
use std::sync::Arc;
use tokio;

async fn get_access_token() -> Result<Arc<Token>, Box<dyn std::error::Error>> {
    let provider = provider().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = provider.token(scopes).await?;
    Ok(token)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match get_access_token().await {
        Ok(token) => {
            println!("{}", token.as_str());
            return Ok(());
        },
        Err(err) => {
            println!("Error: {err:?}");
            return Err(err);
        }
    }
}