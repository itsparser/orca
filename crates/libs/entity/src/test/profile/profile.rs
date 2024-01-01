//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "profile")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,

    #[serde(skip_deserializing)]
    pub app_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::data::Entity")]
    Data,

    #[sea_orm(
        belongs_to = "crate::app::app::Entity",
        from = "Column::AppId",
        to = "crate::app::app::Column::Id"
    )]
    App,
}

// `Related` trait has to be implemented by hand
impl Related<super::data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Data.def()
    }
}

impl Related<crate::app::app::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::App.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
