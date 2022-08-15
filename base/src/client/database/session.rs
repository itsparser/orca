use std::borrow::Borrow;
use sea_orm::{DatabaseConnection, DatabaseTransaction, DbErr, TransactionTrait};
use crate::error::{InternalResult, OrcaError};

pub struct OrcaSession {
    pub conn: DatabaseConnection,
    tx: Option<Box<DatabaseTransaction>>
}

impl OrcaSession {
    pub async fn new(conn: DatabaseConnection) -> InternalResult<OrcaSession> {
        Ok(Self { conn, tx: None })
    }
    pub async fn begin(&mut self) -> InternalResult<&DatabaseTransaction> {
        if self.tx.is_none() {
            self.tx = Some(Box::new(self.conn.begin().await?));
        }
        return Ok(self.tx.as_ref().unwrap().borrow());
    }
    pub async fn tx(&self) -> InternalResult<&DatabaseTransaction> {
        if self.tx.is_none() {
            print!("Transaction is not started");
            return Err(OrcaError::InternalServerError("Transaction is not started".to_string()));
        }
        return Ok(self.tx.as_ref().unwrap().borrow());
    }
    pub async fn commit(&mut self) -> InternalResult<bool> {
        if self.tx.is_some() {
            let tx = self.tx.unwrap().commit().await?;
            self.tx = None;
            return Ok(true);
        }
        Ok(false)
    }
}