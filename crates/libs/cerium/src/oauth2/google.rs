use oauth2::{AuthUrl, TokenUrl};
use reqwest::Error;

use crate::error::CeriumResult;

use super::OauthClient;

impl OauthClient for GoogleOauth2Client {
    fn auth_endpoint(&self) -> CeriumResult<AuthUrl> {
        Ok(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?)
    }

    fn token_endpoint(&self) -> CeriumResult<TokenUrl> {
        Ok(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?)
    }

    fn get_auth_url(&self) -> CeriumResult<String> {
        let mut url = "https://accounts.google.com/o/oauth2/v2/auth?".to_string();
        url.push_str(&format!("client_id={}&", self.client_id));
        url.push_str(&format!("redirect_uri={}&", self.redirect_uri.clone().unwrap()));
        url.push_str("response_type=code&");
        url.push_str("scope=");
        for scope in &self.scopes {
            url.push_str(&format!("{},", scope));
        }
        Ok(url)
    }

    /// Gets an access token from the Google OAuth 2.0 API using the provided authorization code.
    ///
    /// This function sends a POST request to the Google OAuth 2.0 API's token endpoint with the given
    /// code, client ID, and redirect URI. The response is then parsed as JSON to extract the access token.
    async fn get_access_token(&self, code: &str) -> Result<String, Error> { 
        let client = reqwest::Client::new();
        let redirect_uri = self.redirect_uri.clone().unwrap();
        let mut form = std::collections::HashMap::new();
        form.insert("code", code);
        form.insert("client_id", &self.client_id);
        form.insert("client_secret", &self.client_secret);
        form.insert("redirect_uri", redirect_uri.as_str());
        form.insert("grant_type", "authorization_code");
        let res = client.post("https://oauth2.googleapis.com/token")
            .form(&form)
            .send()
            .await?;
        let body = res.text().await?;
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        Ok(json["access_token"].as_str().unwrap().to_string())
    }
}

struct GoogleOauth2Client {
    client_id: String,
    client_secret: String,
    redirect_uri: Option<String>,
    scopes: Vec<String>,
}

impl GoogleOauth2Client {
    pub fn new(client_id: String, client_secret: String, scopes: Vec<String>,
               redirect_uri: Option<String>) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            scopes,
        }
    }
}
