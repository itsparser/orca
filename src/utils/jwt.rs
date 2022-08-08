use std::time::SystemTime;

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::error::{InternalResult, OrcaError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct JWTClaim {
    pub(crate) iss: String,
    pub(crate) sub: String,
    pub(crate) aud: String,
    pub(crate) exp: usize,
    pub(crate) nbf: usize,
    pub(crate) iat: usize,
    pub(crate) jti: String,
    pub(crate) company: String,
    pub(crate) payload: Option<serde_json::Value>,
}



impl JWTClaim {
    /// this will generate the new JWTClaim
    pub(crate) fn new(sub: String, aud: String, exp: Duration, jti: Option<String>,
                      payload: Option<serde_json::Value>) -> Self {
        let current_time = Utc::now();
        let exp = (current_time + exp).timestamp();
        let iat = current_time.timestamp() as usize;
        let _jti = jti.unwrap_or(Uuid::new_v4().to_string());
        Self {
            iss: "Orca".parse().unwrap(),
            sub, aud, exp: (exp as usize), iat, nbf: iat, jti: _jti,
            company: "Orca".parse().unwrap(), payload
        }
    }
    pub(crate) fn decode(jwt: &String) -> InternalResult<Self> {
        let result = decode::<JWTClaim>(&jwt,
                                        &DecodingKey::from_secret("SECRET".as_ref()),
                                        &Validation::default()).map_err(|data| OrcaError::JWTError(data))?;
        Ok(result.claims)
    }

    /// encode the jwt with the secret
    pub(crate) fn encode(&self) -> InternalResult<String> {
        let secret: &[u8] = "SECRET".as_ref();
        let token = encode(&Header::default(), self,
                           &EncodingKey::from_secret(secret)).map_err(|data| OrcaError::JWTError(data))?;
        Ok(token)
    }
}