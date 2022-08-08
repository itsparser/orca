use std::error::Error;
use sea_orm::{Database as SeaDB, DatabaseConnection, DatabaseTransaction, TransactionTrait};
pub mod transaction;
mod session;
mod database;


/// Database -is object ot hold all the databased activity for Orca functionality
///
#[derive(Debug, Clone)]
pub struct Database {
    connection_url: String,
    pub conn: DatabaseConnection,
}

impl Database {
    pub async fn new(connection_url: String) -> Self {
        let conn = SeaDB::connect(&connection_url).await.unwrap();
        Database {
            connection_url,
            conn,
        }
    }

    pub async fn begin_tx(&self) -> &DatabaseTransaction {
        let tx = self.conn.begin().await;
    }

    pub async fn find_by_id<T, R>(&self, entity: &T, id: i32) -> Result<R, Box<dyn Error>>
    where
        T: sea_orm::EntityTrait,
        R: sea_orm::ActiveModelTrait,
    {
        self.conn.begin()
        entity::find_by_id(&self.conn, entity, id).await
    }
}

#[cfg(test)]
mod test_database {
    use futures::executor;
    use crate::client::database::Database;

    #[test]
    fn create() {
        let conn  = Database::new("postgresql://postgres:postgres@localhost:5432/postgres".to_string());
        let b = executor::block_on(conn);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
