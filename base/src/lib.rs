pub mod client;
pub mod error;
pub mod constant;
pub mod config;

use lazy_static::lazy_static;

pub use crate::config::key::Config as KeyConfig;

lazy_static! {
    pub static ref CONFIG: KeyConfig = KeyConfig::new().unwrap();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
