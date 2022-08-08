use futures::executor;
use sea_orm::{Database, DatabaseTransaction, TransactionTrait};
use crate::CLIENT;
use crate::core::CONFIG;
use crate::core::error::{InternalResult, OrcaError};

#[derive(Debug)]
pub struct Transaction {
    tx: DatabaseTransaction,
}

impl Transaction {

    /// Begin transaction for any request in Orca
    pub async fn start() -> InternalResult<Self> {
        let conn = executor::block_on(Database::connect(CONFIG.database.url.clone())).unwrap();
        let tx = conn.begin().await.map_err(|data| OrcaError::DBError(data))?;
        Ok(Self { tx })
    }
    
    pub async fn commit(self) -> InternalResult<()> {
        let result = self.tx.commit().await.map_err(|data| OrcaError::DBError(data))?;
        Ok(result)
    }
    
    pub async fn rollback(self) -> InternalResult<()> {
        let result = self.tx.rollback().await.map_err(|data| OrcaError::DBError(data))?;
        Ok(result)
    }
}