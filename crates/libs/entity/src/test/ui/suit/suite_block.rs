//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(Some(15))",
    enum_name = "block_kind"
)]
pub enum BlockKind {
    #[sea_orm(string_value = "Loop")]
    Loop,
    #[sea_orm(string_value = "Condition")]
    Condition,
    #[sea_orm(string_value = "Reference")]
    Reference,
    // #[sea_orm(string_value = "ValidationGroup")]
    // ValidationGroup,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(Some(15))",
    enum_name = "suit_block_type"
)]
pub enum SuiteBlockType {
    #[sea_orm(string_value = "TestCase")]
    TestCase,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "suite_block")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[serde(skip_deserializing)]
    pub execution_order: i32,
    #[sea_orm(column_name = "type")]
    pub type_field: SuiteBlockType,
    pub reference: Option<Uuid>,


    #[serde(skip_deserializing)]
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    pub description: Option<String>,


    #[serde(skip_deserializing)]
    pub suite_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(belongs_to = "Entity", from = "Column::Id", to = "Column::ParentId")]
    // SelfReferencing,
    #[sea_orm(
        belongs_to = "super::suite::Entity",
        from = "Column::SuiteId",
        to = "super::suite::Column::Id"
    )]
    Suite,
}

// // `Related` trait has to be implemented by hand
// impl Related<super::data_binding::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::DataBinding.def()
//     }
// }

impl Related<super::suite::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Suite.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
