use std::error::Error;
use sea_orm::{Database as SeaDB, DatabaseConnection, DatabaseTransaction, TransactionTrait};
pub mod transaction;
mod session;
mod database;


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
