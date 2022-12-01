use sea_orm::{Database as SeaDB, DatabaseConnection, DatabaseTransaction, TransactionTrait};
use std::error::Error;
mod database;
mod session;
pub mod transaction;

#[cfg(test)]
mod test_database {
    use crate::client::database::Database;
    use futures::executor;

    #[test]
    fn create() {
        let conn =
            Database::new("postgresql://postgres:postgres@localhost:5432/postgres".to_string());
        let b = executor::block_on(conn);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
