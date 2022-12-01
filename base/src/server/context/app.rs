use crate::OrcaConnection;
use std::sync::Arc;

pub struct ApplicationContext {
    conn: Arc<OrcaConnection>,
}

impl ApplicationContext {
    pub fn new(conn: OrcaConnection) -> Self {
        Self {
            conn: Arc::new(conn),
        }
    }
    pub fn conn(&self) -> &Arc<OrcaConnection> {
        &self.conn
    }
}
