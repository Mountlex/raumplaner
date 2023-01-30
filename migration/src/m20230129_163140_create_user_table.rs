use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                extension::postgres::Type::create()
                    .as_enum(UserRoleType::RoleType)
                    .values([UserRoleType::Admin, UserRoleType::DefaultUser])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::UserName)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(
                        ColumnDef::new(User::Role)
                            .enumeration(
                                UserRoleType::RoleType,
                                [UserRoleType::Admin, UserRoleType::DefaultUser],
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::FullName).string().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(User::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(UserRoleType::RoleType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    UserName,
    Password,
    Role,
    FullName,
}

#[derive(Iden)]
enum UserRoleType {
    RoleType,
    Admin,
    DefaultUser,
}
