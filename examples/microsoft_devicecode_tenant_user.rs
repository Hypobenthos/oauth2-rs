use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::StandardDeviceAuthorizationResponse;
use oauth2::{AuthUrl, ClientId, DeviceAuthorizationUrl, Scope, TokenUrl};

use std::error::Error;

// Reference: https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code
// Please use your tenant id when using this example
const TENANT_ID: &str = "{tenant}";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = BasicClient::new(ClientId::new("client_id".to_string()))
        .set_auth_url(AuthUrl::new(format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
            TENANT_ID
        ))?)
        .set_token_url(TokenUrl::new(format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
            TENANT_ID
        ))?)
        .set_device_authorization_url(DeviceAuthorizationUrl::new(format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/devicecode",
            TENANT_ID
        ))?);

    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()
        .add_scope(Scope::new("read".to_string()))
        .request_async(async_http_client)
        .await?;

    eprintln!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        details.verification_uri(),
        details.user_code().secret(),
    );

    let token_result = client
        .exchange_device_access_token(&details)
        .request_async(async_http_client, tokio::time::sleep, None)
        .await;

    eprintln!("Token:{:?}", token_result);

    Ok(())
}
