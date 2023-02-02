mod auth;
mod config;
mod errors;

use std::net::SocketAddr;

use auth::{Claims, Role};
use axum::{
    extract::{Extension, State},
    routing::{get, post},
    Json, Router,
};

use errors::CustomError;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use entity::room;
use tower_http::cors::CorsLayer;

use crate::auth::authorize;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let config = config::Config::new();

    let conn = config.create_connection().await;
    Migrator::up(&conn, None).await.unwrap();
    let state = AppState { conn: conn.clone() };

    // build our application with a route
    let app = Router::new()
        .route("/login", post(authorize))
        .route("/rooms", get(rooms))
        .layer(Extension(config))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

async fn rooms(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<serde_json::Value>>, CustomError> {
    if claims.role == Role::Admin {
        let mzh_rooms: Vec<serde_json::Value> = room::Entity::find()
            .filter(room::Column::Name.contains("MZH"))
            .order_by_asc(room::Column::Name)
            .into_json()
            .all(&state.conn)
            .await?;

        Ok(mzh_rooms.into())
  } else {
      Err(CustomError::Authorization("not authorized!".into()))
  }
}
