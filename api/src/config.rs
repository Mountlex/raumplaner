use std::env;

use sea_orm::{Database, DatabaseConnection};

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Config {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        Config { database_url }
    }

    pub async fn create_connection(&self) -> DatabaseConnection {
        let db: DatabaseConnection = Database::connect(&self.database_url)
            .await
            .expect("Could not connect to database");
        db
    }
}
