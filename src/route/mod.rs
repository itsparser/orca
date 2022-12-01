use actix_web::{web, Responder};

use crate::core::utils::request::generate_success_response;

pub(crate) mod action;
pub(crate) mod admin;
pub(crate) mod auth;
pub(crate) mod profile;
pub(crate) mod table;
pub(crate) mod tst;
pub(crate) mod ws;

/// general_config - this will register all the endpoint in common route
pub fn general_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").route("", web::get().to(health)));
}

async fn health() -> impl Responder {
    generate_success_response(None, None, None)
}
