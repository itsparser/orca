use lazy_static::lazy_static;

pub use crate::core::config::key::Config as KeyConfig;

pub mod client;
pub mod config;
pub mod constant;
pub mod env;
pub mod error;
pub mod middleware;
pub mod server;
pub mod utils;

lazy_static! {
    pub static ref CONFIG: KeyConfig = KeyConfig::new().unwrap();
}
