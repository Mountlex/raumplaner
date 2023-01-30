use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        //manager.create_type(extension::postgres::Type::create().as_enum(UserRoleType::Role).values([UserRoleType::Admin, UserRoleType::DefaultUser]).to_owned()).await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::Role).enumeration(UserRoleType::Role, [UserRoleType::Admin, UserRoleType::DefaultUser]).not_null())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .drop_table(Table::drop().table(User::Table).to_owned())
        .await?;
        
        //manager.drop_type(Type::drop().if_exists().cascade().restrict().name(UserRoleType::Role).to_owned()).await?;
        

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    PasswordHash,
    Role,
    Name
}


#[derive(Iden)]
enum UserRoleType {
    Role,
    Admin,
    DefaultUser
}
