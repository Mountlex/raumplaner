mod config;
mod errors;

use std::net::SocketAddr;

use axum::{
    extract::{Extension, State},
    routing::get,
    Json, Router,
};
use axum_login::{axum_sessions::{async_session::MemoryStore, SessionLayer},
 AuthLayer, PostgresStore, AuthUser, secrecy::SecretVec};
use errors::CustomError;
use migration::{Migrator, MigratorTrait};
use rand::Rng;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use entity::room;
use entity::user;
use room::Entity as Room;

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let conn = config.create_connection().await;
    Migrator::up(&conn, None).await.unwrap();
    let state = AppState { conn: conn.clone() };

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

    let pool = sqlx::postgres::PgPoolOptions::new().connect(&config.database_url).await.unwrap();
    let user_store = PostgresStore::<user::Model>::new(pool);
    let auth_layer = AuthLayer::new(user_store, &secret);

    // build our application with a route
    let app = Router::new()
        .route("/rooms", get(rooms))
        .layer(Extension(config))
        .with_state(state);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}




async fn rooms(State(state): State<AppState>) -> Result<Json<Vec<serde_json::Value>>, CustomError> {
    let mzh_rooms: Vec<serde_json::Value> = Room::find()
        .filter(room::Column::Name.contains("MZH"))
        .order_by_asc(room::Column::Name)
        .into_json()
        .all(&state.conn)
        .await?;

    Ok(mzh_rooms.into())
}
