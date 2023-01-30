mod auth;
mod config;
mod errors;

use std::net::SocketAddr;

use auth::{AuthContext, User};
use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer,
};
use errors::CustomError;
use migration::{Migrator, MigratorTrait};
use rand::Rng;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use entity::room;

use crate::auth::{RequireAuth, Role, SeaUserStore};

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let conn = config.create_connection().await;
    Migrator::up(&conn, None).await.unwrap();
    let state = AppState { conn: conn.clone() };

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

    let user_store = SeaUserStore::new(&conn);
    let auth_layer = AuthLayer::new(user_store, &secret);

    // build our application with a route
    let app = Router::new()
        .route(
            "/protected",
            get(protected_handler).layer(RequireAuth::login()),
        )
        .route(
            "/protected_admin",
            get(admin_handler).layer(RequireAuth::login_with_role(Role::Admin..)), // At least `Admin`.
        )
        .route(
            "/protected_user",
            get(user_handler).layer(RequireAuth::login_with_role(Role::DefaultUser..)), // At least `User`.
        )
        .route("/login", get(login_handler))
        .route("/logout", get(logout_handler))
        .route("/rooms", get(rooms))
        .layer(auth_layer)
        .layer(session_layer)
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

async fn login_handler(mut auth: AuthContext) {
    // TODO
}

async fn logout_handler(mut auth: AuthContext) {
    dbg!("Logging out user: {}", &auth.current_user);
    auth.logout().await;
}

async fn protected_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    format!("Logged in as: {}", user.name)
}

async fn admin_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    format!("Admin logged in as: {}", user.name)
}

async fn user_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    format!("Admin or user logged in as: {}", user.name)
}

async fn rooms(State(state): State<AppState>) -> Result<Json<Vec<serde_json::Value>>, CustomError> {
    let mzh_rooms: Vec<serde_json::Value> = room::Entity::find()
        .filter(room::Column::Name.contains("MZH"))
        .order_by_asc(room::Column::Name)
        .into_json()
        .all(&state.conn)
        .await?;

    Ok(mzh_rooms.into())
}
