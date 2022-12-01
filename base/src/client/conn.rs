use crate::error::InternalResult;
use sea_orm::{Database as SeaDB, DatabaseConnection};
use std::rc::Rc;
use std::sync::Arc;
use thirtyfour::{Capabilities, WebDriver};

pub struct Conn;

impl Conn {
    pub async fn database() -> InternalResult<DatabaseConnection> {
        Ok(SeaDB::connect(crate::CONFIG.database.url.clone()).await?)
    }
    pub async fn webdriver<C>(capabilities: C) -> InternalResult<WebDriver>
    where
        C: Into<Capabilities>,
    {
        Ok(WebDriver::new("http://localhost:4444", capabilities).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Connection {
    pub database: Option<Arc<DatabaseConnection>>,
}

impl Connection {
    pub fn new() -> Self {
        Self { database: None }
    }

    pub async fn database(mut self) -> InternalResult<Arc<DatabaseConnection>> {
        if self.database.is_none() {
            self.database = Some(Arc::new(
                SeaDB::connect(crate::CONFIG.database.url.clone()).await?,
            ));
        }
        Ok(self.database.unwrap())
    }
    pub async fn webdriver<C>(capabilities: C) -> InternalResult<WebDriver>
    where
        C: Into<Capabilities>,
    {
        Ok(WebDriver::new("http://localhost:4444", capabilities).await?)
    }
}
