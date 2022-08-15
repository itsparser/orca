use std::error::Error;
use sea_orm::{Database as SeaDB, DatabaseConnection, DatabaseTransaction, TransactionTrait};
use crate::error::InternalResult;

/// Database -is object ot hold all the databased activity for Orca functionality
///
#[derive(Debug, Clone)]
pub struct Database<'db, 'conn> {
    connection_url: &'db String,
    pub conn: &'conn DatabaseConnection,
}

impl<'db, 'conn> Database<'db, 'conn> {
    // pub async fn new(connection_url: String) -> InternalResult<Self> {
    //     let conn: DatabaseConnection = SeaDB::connect(&connection_url).await?;
    //     let db = Database {
    //         connection_url: &connection_url,
    //         conn: &conn,
    //     };
    //     Ok(db)
    // }

    // pub async fn begin_tx(&self) -> &DatabaseTransaction {
    //     let tx = self.conn.begin().await;
    // }

    // pub async fn find_by_id<T, R>(&self, entity: &T, id: i32) -> Result<R, Box<dyn Error>>
    // where
    //     T: sea_orm::EntityTrait,
    //     R: sea_orm::ActiveModelTrait,
    // {
    //     self.conn.begin()
    //     entity::find_by_id(&self.conn, entity, id).await
    // }
}