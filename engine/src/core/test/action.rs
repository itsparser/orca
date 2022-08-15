use entity::test_action;
use sea_orm::{ConnectionTrait, Paginator, SelectorTrait};
use sea_orm::{query::*};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use thirtyfour::{By, WebDriver};
use thirtyfour::prelude::WebDriverResult;

use base::{error::{InternalResult, OrcaError}};

use crate::server::context::EngineContext;

pub struct Action<'action> {
    pub ctx: &'action EngineContext<'action>
}

impl<'action> Action<'action> {
    /// this will create new Action Step Object that will perform sequence of object
    pub fn new(ctx: &'action EngineContext<'action>) -> Self {
        Self { ctx }
    }

    pub async fn dispatch(&mut self, action: &test_action::Model) -> InternalResult<()> {
        match action.command.as_str() {
            "open" => self.click(action).await?,
            "click" => self.click(action).await?,
            "doubleClick" => self.click(action).await?,
            "select" => self.click(action).await?,
            "type" => self.click(action).await?,
            _ => unimplemented!(),
        };
        Ok(())
    }

    async fn selector(&mut self, action: &test_action::Model) -> By {
        let _action = action.clone();
        let x = _action.target.as_str();
        match _action.selector.unwrap().as_str() {
            "Id" => By::Id(x),
            "XPath" => By::XPath(x),
            "LinkText" => By::LinkText(x),
            // "PartialLinkText" => By::PartialLinkText(x),
            "Name" => By::Name(x),
            "Tag" => By::Tag(x),
            "ClassName" => By::ClassName(x),
            "Css" => By::Css(x),
            _ => By::Id(x)
        }
    }

    async fn open(self, url: String) -> InternalResult<()> {
        self.ctx.driver.goto(url).await.map_err(|data| OrcaError::WebDriverError(data))?;
        Ok(())
    }

    async fn click(&mut self, action: &test_action::Model) -> InternalResult<()> {
        let selector = self.selector(action).await;
        let elem_button = self.ctx.driver.find(selector).await
            .map_err(|data| OrcaError::WebDriverError(data))?;
        elem_button.click().await.map_err(|data| OrcaError::WebDriverError(data))?;
        Ok(())
    }
}


#[cfg(test)]
mod test_actions {
    #[test]
    fn create() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
