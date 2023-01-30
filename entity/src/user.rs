//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use super::sea_orm_active_enums::RoleType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_name: String,
    pub password: String,
    pub role: RoleType,
    pub full_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}