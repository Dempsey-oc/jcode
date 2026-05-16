//! Azure credential helpers built on `azure_identity`. Used by the Azure OpenAI
//! provider and any other crate that needs an AAD bearer token.

use anyhow::Result;
use azure_core::credentials::TokenCredential;

pub async fn get_bearer_token(scope: &str) -> Result<String> {
    let credential = azure_identity::DefaultAzureCredential::new()?;
    let token = credential.get_token(&[scope]).await?;
    Ok(token.token.secret().to_string())
}
