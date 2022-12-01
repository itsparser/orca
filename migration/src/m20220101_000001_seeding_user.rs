use entity::sea_orm::ActiveValue::Set;
use entity::{audit_log, profile, profile_data, role, role_scope, user, user_role, user_session};
use sea_orm::ActiveModelTrait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_seeding_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        user::ActiveModel {
            first_name: Set("Admin".to_owned()),
            last_name: Set(Some("".to_owned())),
            name: Set("Admin".to_owned()),
            email: Set("admin@orca.ci".to_owned()),
            password: Set(Some("admin@123".to_owned())),
            is_active: Set(true.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await
    }
}
