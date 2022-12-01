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
    println!("print 1");
    let db_conn = Conn::database().await?;
    println!("print 2");
    let mut session = OrcaSession::new(db_conn).await?;
    println!("print 3");
    let mut context = EngineContext::new(&mut session, &drive);
    println!("print 4");
    let mut case_instance = Case::new(&context);
    let case = case_instance.run(1).await?;
    println!("End Engine");
    Ok(())
}
