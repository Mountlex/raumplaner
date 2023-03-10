pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230129_163140_create_user_table;
mod m20230130_142703_seed;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230129_163140_create_user_table::Migration),
            Box::new(m20230130_142703_seed::Migration),
        ]
    }
}
