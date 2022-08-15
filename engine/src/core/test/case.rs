use entity::test_action;
use thirtyfour::WebDriver;
use base::error::{InternalResult, OrcaError};
use crate::server::context::EngineContext;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::Set;
use sea_orm::{entity::*, query::*};
use crate::core::test::action::Action;

pub struct Case<'case> {
    pub ctx: &'case EngineContext<'case>
}

impl<'case> Case<'case> {
    pub fn new(ctx: &'case EngineContext<'case>) -> Self {
        Self { ctx }
    }
    pub async fn run(&mut self, case_id: i64) -> InternalResult<()> {
        let mut action_instance = Action::new(self.ctx);
        let mut result_actions = test_action::Entity::find()
            .filter(test_action::Column::TestCaseId.eq(case_id))
            .order_by_asc(test_action::Column::ExecutionOrder)
            .paginate(&self.ctx.session.conn, 50);
        while let Some(actions) = result_actions.fetch_and_next().await? {
            // Do something on actions: Vec<test_action::Model>
            for action in actions {
                action_instance.dispatch(&action).await?;
            }
        }
        Ok(())
    }

}

