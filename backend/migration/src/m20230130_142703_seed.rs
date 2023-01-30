use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set},
};

use entity::{sea_orm_active_enums::RoleType, user};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        user::ActiveModel {
            user_name: Set("admin".into()),
            password: Set("123".into()),
            role: Set(RoleType::Admin),
            full_name: Set("Der Admin".into()),
        }
        .insert(db)
        .await?;

        user::ActiveModel {
            user_name: Set("user".into()),
            password: Set("123".into()),
            role: Set(RoleType::DefaultUser),
            full_name: Set("Der User".into()),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let admin = user::Entity::find_by_id("admin".into()).one(db).await?;
        let user = user::Entity::find_by_id("user".into()).one(db).await?;
        user.unwrap().delete(db).await?;
        admin.unwrap().delete(db).await?;

        Ok(())
    }
}
