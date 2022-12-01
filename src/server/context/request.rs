use std::borrow::Borrow;
use std::future::Future;
use std::pin::Pin;
use std::process::Output;
use std::sync::Arc;

use actix::prelude::*;
use actix_http::{header, HttpMessage, Payload};
use actix_web::dev::ServiceRequest;
use actix_web::{FromRequest, HttpRequest};
use base::client::conn::Connection;
use base::OrcaConnection;
use entity::user::User;
use entity::user_session::UserSession;
use entity::{user, user_session};
use futures::executor;
use http::header::HeaderName;
use http::HeaderValue;
use log::error;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, JsonValue,
    QueryFilter, QueryOrder, Set,
};

use crate::constant::endpoint::PUBLIC_ENDPOINT;
use crate::core::client::{database, Client, CLIENT};
use crate::core::error::{InternalResult, OrcaError};
use crate::utils::jwt::JWTClaim;

// This struct represents state
#[derive(Debug, Clone, Default)]
pub struct RequestContext<'rc> {
    auth_token: Option<&'rc String>,
    request_id: Option<&'rc String>,
    jwt_session: Option<&'rc JWTClaim>,
    user: Option<&'rc User>,
    session: Option<&'rc UserSession>,
}

impl<'rc> FromRequest for RequestContext<'rc> {
    type Error = OrcaError;
    type Future = Pin<Box<dyn Future<Output = Result<RequestContext<'rc>, Self::Error>>>>;
    fn from_request(_req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // let auth_token = req.headers().get(header::AUTHORIZATION).map(|h| h.to_str().unwrap().to_string());
        let req = _req.clone();
        let future = async move {
            let rc = req
                .extensions_mut()
                .get_mut::<RequestContext>()
                .map(|c| c.clone())
                .unwrap();
            Ok(rc)
        };
        Box::pin(future)
    }
}

/// RequestContext - will have all the dependency for the request
/// this will get created on each request and Will Construct required object in lazy
impl<'rc> RequestContext<'rc> {
    /// Create a new RequestContext
    pub fn new(request: &ServiceRequest) -> InternalResult<Self> {
        let mut _self = Self {
            request_id: None,
            jwt_session: None,
            user: None,
            auth_token: None,
            session: None,
        };
        let uri = request.uri().path();
        log::info!("Request URI - {}", uri);
        if !PUBLIC_ENDPOINT.iter().any(|item| uri.contains(item)) {
            executor::block_on(_self.authorize_request(request))?;
        }
        Ok(_self)
    }

    pub fn get_header_value(
        &self,
        request: &ServiceRequest,
        header_name: &HeaderName,
    ) -> Option<String> {
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
        // self.auth_token = Some(&api_key);
        return true;
    }

    async fn cookie_based_authentication(
        &mut self,
        cookie_value: &'rc String,
    ) -> InternalResult<bool> {
        // self.auth_token = Some(cookie_value);
        // log::debug!("Cookie Value - {}", self.auth_token.clone().unwrap());
        // let jwt_claim :JWTClaim = JWTClaim::decode(&self.auth_token.clone().unwrap())?;
        // self.jwt_session = Some(jwt_claim.clone().borrow());
        // let payload = jwt_claim.payload.unwrap();
        // let id = payload.get("id").unwrap().as_i64().unwrap() as i32;
        // let session_id = jwt_claim.jti;
        // log::info!("Logged in User - {}, session_id - {}", id.clone(), session_id.clone());
        // let _user = user::Entity::find()
        //             .filter(Condition::all()
        //                 .add(user::Column::Id.eq(id))
        //                 .add(user::Column::IsActive.eq(true))
        //             ).one(self.database()).await
        //             .map_err(|data| OrcaError::DBError(data))?;
        // let _session = user_session::Entity::find()
        //             .filter(user_session::Column::SessionId.eq(session_id)).one(self.database()).await
        //             .map_err(|data| OrcaError::DBError(data))?;

        // if _user.is_none() || _session.is_none() {
        //     return Err(OrcaError::UnAuthenticated);
        // }
        // self.user = Some(User::from_active_model(_user.unwrap().into()));
        // self.session = Some(UserSession::from_active_model(_session.unwrap().into()));
        return Ok(true);
    }

    async fn authorize_request(&mut self, request: &ServiceRequest) -> InternalResult<bool> {
        let auth_token = self.get_header_value(request, &header::AUTHORIZATION);
        let cookie = self.get_cookie(request, "_OUSI_".to_string());
        // if (auth_token.is_some() && self.api_based_authentication(auth_token.unwrap())) ||
        //     (cookie.is_some() && self.cookie_based_authentication(cookie.unwrap().clone()).await?) {
        //     log::info!("API Value - Success");
        //     return Ok(true);
        // }
        // log::info!("Auth Value - Failed");
        // return Ok(false);
        Ok(true)
    }

    pub fn validate_request(&self, request: &ServiceRequest) -> Result<(), String> {
        let auth_token = self.get_header_value(request, &header::AUTHORIZATION);
        let auth = request
            .headers()
            .get(header::AUTHORIZATION)
            .ok_or(String::from("No Authorization header found"))?;
        let auth_str = auth
            .to_str()
            .map_err(|_| String::from("Authorization header is not a valid string"))?;
        let l_auth_str = auth_str.split_whitespace().collect::<Vec<&str>>();
        let l_auth_str_len = l_auth_str
            .last()
            .ok_or(String::from("Authorization header is not a valid string"))?;
        log::info!("Authorization header: {}", l_auth_str_len);
        Ok(())
    }
    pub fn database(&mut self) -> DatabaseConnection {
        // if self.db.is_none() {
        //     self.db = Some(CLIENT.lock().unwrap().clone().database());
        // }
        // self.db.unwrap()
        CLIENT.lock().unwrap().clone().database()
    }
}
