use std::borrow::Borrow;
use sea_orm::{DatabaseConnection, DatabaseTransaction, DbErr, TransactionTrait};
use crate::error::InternalResult;

pub struct OrcaSession {
    pub conn: DatabaseConnection,
    tx: Option<Box<DatabaseTransaction>>,
}

impl OrcaSession {
    pub async fn new(conn: DatabaseConnection) -> InternalResult<OrcaSession> {
        Ok(Self { conn, tx: None })
    }
    pub async fn begin(&mut self) -> InternalResult<&DatabaseTransaction> {
        if self.tx.is_none() {
            let tx = Box::new(self.conn.begin().await?);
            self.tx = Some(tx);
        }
        return Ok(self.tx.as_ref().unwrap().borrow());
    }
}