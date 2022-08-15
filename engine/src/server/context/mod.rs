use sea_orm::TransactionTrait;
use thirtyfour::WebDriver;
use base::client::database::session::OrcaSession;
use base::error::InternalResult;

pub struct EngineContext<'ctx> {
    pub session: &'ctx OrcaSession,
    pub driver: &'ctx WebDriver,
}

impl<'ctx> EngineContext<'ctx> {
    pub fn new(session: &'ctx OrcaSession, driver: &'ctx WebDriver) -> Self {
        Self { session, driver }
    }
    // pub async fn start_tx(&mut self) -> InternalResult<()> {
    //     self.session.to_owned().begin().await?;
    //     Ok(())
    // }
}