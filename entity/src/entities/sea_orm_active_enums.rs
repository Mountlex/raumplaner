//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role")]
pub enum Role {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "default_user")]
    DefaultUser,
}
