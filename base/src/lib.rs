use lazy_static::lazy_static;

pub use crate::client::conn::Connection as OrcaConnection;
pub use crate::config::key::Config as KeyConfig;

pub mod client;
pub mod config;
pub mod constant;
pub mod error;
pub mod server;
pub mod utils;

lazy_static! {
    pub static ref CONFIG: KeyConfig = KeyConfig::new().unwrap();
    pub static ref Connection: OrcaConnection = OrcaConnection::new();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
