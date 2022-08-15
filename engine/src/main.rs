use sea_orm::TransactionTrait;
use base::client::conn::Conn;
use base::client::database::session::OrcaSession;
use base::client::selenium::OrcaWebDriver;
use base::error::InternalResult;
use engine::core::test::case::Case;
use engine::server::context::EngineContext;


#[tokio::main]
async fn main() -> InternalResult<()> {
    println!("Start Engine");
    let drive = OrcaWebDriver::default().await.unwrap();
    let db_conn = Conn::database().await?;
    let mut session = OrcaSession::new(db_conn).await?;
    let mut context = EngineContext::new(&session, &drive);
    let mut case_instance = Case::new(&context);
    let case = case_instance.run(45);
    println!("End Engine");
    Ok(())
}
