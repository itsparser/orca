use std::future::Future;
use actix::prelude::*;
use std::pin::Pin;
use std::process::Output;
use actix_http::{header, HttpMessage, Payload};
use actix_web::dev::ServiceRequest;
use actix_web::{FromRequest, HttpRequest};
use http::header::HeaderName;
use http::HeaderValue;
use log::error;
use crate::constant::endpoint::PUBLIC_ENDPOINT;

use crate::core::client::{CLIENT, Client, database};
use crate::core::client::database::Database;
use crate::core::error::{InternalResult, OrcaError};
use crate::utils::jwt::JWTClaim;

// This struct represents state
#[derive(Debug, Clone, Default)]
pub struct RequestContext {
    db: Option<Database>,
    auth_token: Option<String>,
    request_id: Option<String>
}

impl FromRequest for RequestContext {
    type Error = OrcaError;
    type Future = Pin<Box<dyn Future<Output=Result<RequestContext, Self::Error>>>>;
    fn from_request(_req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // let auth_token = req.headers().get(header::AUTHORIZATION).map(|h| h.to_str().unwrap().to_string());
        let req = _req.clone();
        let future = async move {
            let rc = req.extensions_mut().get_mut::<RequestContext>().map(|c| c.clone()).unwrap();
            Ok(rc)
        };
        Box::pin(future)
    }
}

/// RequestContext - will have all the dependency for the request
/// this will get created on each request and Will Construct required object in lazy
impl RequestContext {
    /// Create a new RequestContext
    pub fn new(request: &ServiceRequest) -> Self {
        let mut _self = Self { db: None, request_id: None, auth_token: None };
        let uri = request.uri().path();
        log::info!("Request URI - {}", uri);
        if !PUBLIC_ENDPOINT.iter().any(|item| uri.contains(item)) {
            // _self.validate_request(request).expect("TODO: panic message");
            _self.authorize_request(request);
        }
        _self
    }

    pub fn get_header_value(&self, request: &ServiceRequest, header_name: &HeaderName) -> Option<String> {
        let header_value = request.headers().get(header_name);
        if header_value.is_some() {
            return Some(header_value.unwrap().to_str().unwrap().to_string());
        }
        return None;
    }

    pub fn get_cookie(&self, request: &ServiceRequest, cookie_name: String) -> Option<String> {
        let cookie = request.cookie(cookie_name.as_str());
        if cookie.is_some() {
            return Some(cookie.unwrap().value().to_string());
        }
        return None;
    }

    fn api_based_authentication(&mut self, api_key: String) -> bool {
        self.auth_token = Some(api_key);
        return true;
    }

    fn cookie_based_authentication(&mut self, cookie_value: String) -> InternalResult<bool> {
        self.auth_token = Some(cookie_value);
        log::debug!("Cookie Value - {}", self.auth_token.clone().unwrap());
        let jwt_claim = JWTClaim::decode(&self.auth_token.clone().unwrap())?;
        let payload = jwt_claim.payload.unwrap();
        let id = payload.get("id").unwrap();
        log::info!("Logged in User - {}", id);
        return Ok(true);
    }

    fn authorize_request(&mut self, request: &ServiceRequest) -> InternalResult<bool> {
        let auth_token = self.get_header_value(request,&header::AUTHORIZATION);
        let cookie = self.get_cookie(request, "_OUSI_".to_string());
        if (auth_token.is_some() && self.api_based_authentication(auth_token.unwrap())) ||
            (cookie.is_some() && self.cookie_based_authentication(cookie.unwrap())?) {
            log::info!("API Value - Success");
            return Ok(true);
        }
        log::info!("Auth Value - Failed");
        return Ok(false);
    }

    pub fn validate_request(&self, request: &ServiceRequest) -> Result<(), String> {
        let auth_token = self.get_header_value(request,&header::AUTHORIZATION);
        let auth = request.headers().get(header::AUTHORIZATION).ok_or(String::from("No Authorization header found"))?;
        let auth_str = auth.to_str().map_err(|_| String::from("Authorization header is not a valid string"))?;
        let l_auth_str = auth_str.split_whitespace().collect::<Vec<&str>>();
        let l_auth_str_len = l_auth_str.last().ok_or(String::from("Authorization header is not a valid string"))?;
        log::info!("Authorization header: {}", l_auth_str_len);
        Ok(())
    }

    pub(crate) fn set_request_value(mut req: ServiceRequest) -> ServiceRequest{
        let rc = Self::default();
        req.extensions_mut().insert(rc);
        req
    }
    pub fn database(&mut self) -> Database {
        if self.db.is_none() {
            self.db = Some(CLIENT.lock().unwrap().clone().database());
        }
        self.db.clone().unwrap()
    }
}

