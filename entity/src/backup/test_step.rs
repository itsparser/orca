//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TestStep {
    pub command: String,
    pub target: String,
    pub value: Option<String>,
    pub output: Option<String>,
    pub desc: Option<String>,
    pub execution_order: i32,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "test_step")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub command: String,
    pub target: String,
    #[sea_orm(nullable)]
    pub value: Option<String>,
    #[sea_orm(nullable)]
    pub output: Option<String>,
    #[sea_orm(nullable)]
    pub desc: Option<String>,
    pub execution_order: i32,
    pub test_case_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::test_case::Entity",
        from = "Column::TestCaseId",
        to = "super::test_case::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TestCase,
}

impl Related<super::test_case::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TestCase.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
