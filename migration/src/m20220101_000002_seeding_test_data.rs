use entity::sea_orm::ActiveValue::Set;
use entity::{
    audit_log, profile, profile_data, role, role_scope, test_action, test_case, user, user_role,
    user_session,
};
use sea_orm::ActiveModelTrait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000002_seeding_test_data"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let t1 = test_case::ActiveModel {
            name: Set("Test Case".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;
        let t2 = test_case::ActiveModel {
            name: Set("Test Case 2".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        test_action::ActiveModel {
            command: Set("Open".to_owned()),
            target: Set("https://www.rust-lang.org/".to_owned()),
            // value: Set("Test Step".to_owned()),
            // output: Set("Test Step".to_owned()),
            execution_order: Set(1),

            test_case_id: Set(t1.id),
            ..Default::default()
        }
        .insert(db)
        .await?;

        // test_action::ActiveModel {
        //     command: Set("Open".to_owned()),
        //     target: Set("https://www.rust-lang.org/".to_owned()),
        //     // value: Set("Test Step".to_owned()),
        //     // output: Set("Test Step".to_owned()),
        //     execution_order: Set(1),
        //
        //     test_case_id: Set(t1.id),
        //     ..Default::default()
        // }.insert(db)
        // .await?;

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
