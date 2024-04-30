use oauth2::{AuthUrl, TokenUrl};

use crate::error::CeriumResult;

mod google;


pub trait OauthClient {
    fn auth_endpoint(&self) -> CeriumResult<AuthUrl>;
    fn token_endpoint(&self) -> CeriumResult<TokenUrl>;
    fn get_auth_url(&self) -> CeriumResult<String>;
    async fn get_access_token(&self, code: &str) -> Result<String, reqwest::Error>;
    fn get_redirect_url(&self) -> String {
        "http://localhost:3000/oauth2/callback".to_string()
    }
}