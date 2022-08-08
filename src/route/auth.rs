use actix_web::{HttpRequest, HttpResponse, Responder, Scope, web};
use actix_web::{cookie::Cookie, web::Path};
use actix_web::cookie::SameSite;
use actix_web::cookie::time::Duration as CookieDuration;
use actix_web::http::header;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use entity::{prelude::*, profile, profile_data, user, user_session};
use entity::user::User;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, JsonValue, QueryFilter, QueryOrder, Set};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::constant::metadata::user::AuthData;
use crate::core::error::{OrcaError, OrcaResult};
use crate::core::utils::request::generate_success_response;
use crate::server::context::request::RequestContext;
use crate::utils::jwt::JWTClaim;

/// profile_config - this will register all the endpoint in profile route
pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signin", web::post().to(signin))
            .route("/google", web::get().to(get_redirection_url))
            .route("/google/callback", web::post().to(signin))
            .route("/resend", web::post().to(signin))
    );

}
/// signin - will get username and password as payload
async fn signin(mut request_ctx: RequestContext, auth_data: web::Json<AuthData>,
                req: HttpRequest) -> OrcaResult {
    let ua = req.headers().get(header::USER_AGENT).unwrap().to_str().unwrap();
    let username = auth_data.clone().username;
    let _user = user::Entity::find()
        .filter(Condition::all()
            .add(user::Column::Email.eq(&*auth_data.username))
            .add(user::Column::IsActive.eq(true))
            .add(user::Column::Password.eq(&*auth_data.password))
        )
        .order_by_asc(user::Column::Name)
        .one(request_ctx.database()).await
        .map_err(|data| OrcaError::DBError(data))?;
    if _user.is_none() {
        return Err(OrcaError::UnAuthenticated);
    }
    let usr = User::from_active_model(_user.unwrap().into());
    let cuser = usr.clone();
    let duration = Duration::days(5);
    let expire_by = (Utc::now() + duration).naive_utc();
    let session_id = Uuid::new_v4().to_string();
    let _session = user_session::ActiveModel {
        user_id: Set(cuser.id.unwrap()),
        session_id: Set(session_id.clone()),
        session_type: Set("Login".parse().unwrap()),
        user_agent: Set(ua.clone().to_string()),
        email: Set(cuser.email),
        expires_by: Set(expire_by),
        ..Default::default()
    }.insert(request_ctx.database()).await.map_err(|data| OrcaError::DBError(data))?;
    let _usr: serde_json::Value = serde_json::from_str(serde_json::to_string(&usr).unwrap().as_str()).unwrap();
    let claim = JWTClaim::new(username, "AUTH".parse().unwrap(),
                              duration, Some(session_id), Some(_usr));
    let jwt = claim.encode()?;
    let cookie = Cookie::build("_OUSI_", jwt).path("/").secure(true)
            .max_age(CookieDuration::days(5)).secure(true)
            .same_site(SameSite::Strict)
            .finish();
    Ok(HttpResponse::Ok().cookie(cookie).json(json!({"status": "success"})))
}

/// get_redirection_url - get the redirection url for the google authentication
async fn get_redirection_url() -> impl Responder {
    let redirection_url = format!("https://");
    HttpResponse::Found().append_header(("location", redirection_url)).finish()
}