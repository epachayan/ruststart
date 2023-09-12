extern crate oauth2;
extern crate reqwest;

use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::Credentials;
use std::env;

fn main() {
    // Replace these values with your ADFS and Azure AD application's information
    let adfs_client_id = "YOUR_ADFS_CLIENT_ID";
    let adfs_client_secret = "YOUR_ADFS_CLIENT_SECRET";
    let azure_client_id = "YOUR_AZURE_CLIENT_ID";
    let azure_client_secret = "YOUR_AZURE_CLIENT_SECRET";
    let adfs_auth_url = "https://your-adfs-server/adfs/oauth2/authorize";
    let adfs_token_url = "https://your-adfs-server/adfs/oauth2/token";
    let azure_auth_url = format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
        "YOUR_AZURE_TENANT_ID"
    );
    let azure_token_url = format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        "YOUR_AZURE_TENANT_ID"
    );

    // Create the ADFS OAuth2 client
    let adfs_client = BasicClient::new(
        Credentials::new(adfs_client_id.to_string(), adfs_client_secret.to_string()),
        oauth2::AuthUrl::new(adfs_auth_url.to_string()).expect("Invalid ADFS authorization endpoint URL"),
        Some(oauth2::TokenUrl::new(adfs_token_url.to_string()).expect("Invalid ADFS token endpoint URL")),
    )
    .set_redirect_url(oauth2::RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"));

    // Try to authenticate locally against ADFS first
    let local_auth_result = perform_local_adfs_authentication(&adfs_client);

    match local_auth_result {
        Ok(token_result) => {
            // Local authentication succeeded
            println!("Local ADFS Authentication Succeeded");
            println!("Access token: {:?}", token_result.access_token());
        }
        Err(_) => {
            // Local authentication failed, try Azure AD authentication
            println!("Local ADFS Authentication Failed. Falling back to Azure AD Authentication");

            // Create the Azure AD OAuth2 client
            let azure_client = BasicClient::new(
                Credentials::new(azure_client_id.to_string(), azure_client_secret.to_string()),
                oauth2::AuthUrl::new(azure_auth_url.to_string()).expect("Invalid Azure AD authorization endpoint URL"),
                Some(oauth2::TokenUrl::new(azure_token_url.to_string()).expect("Invalid Azure AD token endpoint URL")),
            )
            .set_redirect_url(oauth2::RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"));

            // Generate the authorization URL for Azure AD
            let (auth_url, _csrf_token) = azure_client
                .authorize_url(oauth2::CsrfToken::new_random)
                .add_scope("openid".to_string())
                .add_scope("profile".to_string())
                .add_scope("offline_access".to_string())
                .url();

            println!("Please visit this URL to authorize the application against Azure AD: {}", auth_url);

            // Enter the authorization code received after the user grants permission
            println!("Enter the authorization code: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let code = input.trim();

            // Exchange the authorization code for an access token
            let token_result = azure_client
                .exchange_code(code)
                .request(http_client)
                .expect("Failed to exchange code for token");

            // Print the access token
            println!("Access token: {:?}", token_result.access_token());
        }
    }
}

fn perform_local_adfs_authentication(adfs_client: &BasicClient) -> Result<oauth2::TokenResponse, reqwest::Error> {
    // Implement your local ADFS authentication logic here
    // This can involve interacting with ADFS using the ADFS OAuth2 client

    // For example, you can generate an authorization URL and prompt the user to visit it
    let (auth_url, _csrf_token) = adfs_client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope("openid".to_string())
        .add_scope("profile".to_string())
        .add_scope("offline_access".to_string())
        .url();

    println!("Please visit this URL to authorize the application against ADFS: {}", auth_url);

    // Handle the user's interaction and retrieve the authorization code
    // Then exchange the code for an access token and return it
    // You should implement this part based on your ADFS setup

    // For demonstration purposes, we'll return an error to indicate that local authentication failed
    Err(reqwest::Error::from(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Local ADFS authentication failed",
    )))
}
