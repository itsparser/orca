use sea_orm::{Database as SeaDB, DatabaseConnection, DatabaseTransaction, TransactionTrait};
use std::error::Error;

/// Database -is object ot hold all the databased activity for Orca functionality
///
#[derive(Debug, Clone)]
pub struct Database<'db> {
    connection_url: &'db String,
    pub conn: &'db DatabaseConnection,
}

impl<'db> Database<'db> {
    // pub async fn new(connection_url: String) -> Database<'db> {
    //     let conn: DatabaseConnection = SeaDB::connect(&connection_url).await.unwrap();
    //     Database {
    //         connection_url: &connection_url,
    //         conn: &conn,
    //     }
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
