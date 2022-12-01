use crate::core::client::CLIENT;
use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpServer};
use base::OrcaConnection;
use log::LevelFilter;

use crate::core::middleware::{error, request::RequestHandler};
use crate::core::utils::logger;

mod constant;
mod core;
mod route;
mod server;
mod utils;

/// main - Orca Start here
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return bootstrap_application_server().await;
}

/// bootstrap_application_server - will kick start the orca application server for the application
/// this will be responsible for all the application api request in Orca
async fn bootstrap_application_server() -> std::io::Result<()> {
    logger::init().expect("TODO: panic message");
    let oc = OrcaConnection::new();
    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().limit(4096))
            .app_data(oc.clone())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(RequestHandler::default())
            .configure(route::general_config)
            .service(
                web::scope("/v1")
                    .configure(route::auth::auth_config)
                    .configure(route::admin::admin_config)
                    .service(
                        web::scope("/test")
                            .configure(route::profile::profile_config)
                            .service(web::scope("/ui").configure(route::action::test_action_config))
                            .service(
                                web::scope("/api").configure(route::action::test_action_config),
                            )
                            .service(
                                web::scope("load").configure(route::action::test_action_config),
                            ),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
