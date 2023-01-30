use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Room::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Room::Name).string().not_null())
                    .col(ColumnDef::new(Room::Size).integer().not_null())
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Room::Table)
            .columns([Room::Name, Room::Size])
            .values_panic(["MZH 3150".into(), 15.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Room::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Room {
    Table,
    Id,
    Name,
    Size,
}
