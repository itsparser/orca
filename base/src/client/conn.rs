use sea_orm::{Database as SeaDB, DatabaseConnection};
use thirtyfour::{Capabilities, WebDriver};
use crate::error::InternalResult;

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