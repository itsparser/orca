// 1.4.0
use std::sync::{Arc, Mutex};

use futures::executor;
use lazy_static::lazy_static;
use sea_orm::{Database, DatabaseConnection};

pub mod conn;
pub mod database;
pub mod redis;
pub mod selenium;

lazy_static! {
    pub static ref CLIENT: Mutex<Client> = Mutex::new(Client::default());
}

#[derive(Debug, Clone, Default)]
pub struct Client {
    database: Option<Arc<DatabaseConnection>>,
    redis: Option<redis::Redis>,
}

impl Client {
    pub fn default() -> Self {
        Client {
            database: None,
            redis: None,
        }
    }

    // pub fn database(mut self) -> DatabaseConnection {
    //     if self.database.is_none() {
    //         log::info!("Initializing database");
    //         let conn = executor::block_on(Database::connect(super::CONFIG.database.url.clone())).unwrap();
    //         self.database = Some(Arc::new(conn));
    //     }
    //     return self.database.unwrap().clone();
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let cli = super::Client::default();
        // let database = cli.database();
    }
}
