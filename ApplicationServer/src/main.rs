use axum::{extract::State, routing::get, Router};
mod auth;
use auth::auth_service::auth_service::{auth_routers, register_user};

mod email;
use email::email_service::email_service::routers;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[tokio::main]
async fn main() {
    // Init db pool
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://LINKEDU:123@central_user_database:5432/CentralUserDatabase")
        .await
        .unwrap();

    let shared_state = AppState { db_pool: pg_pool };

    let app = Router::new()
        .route("/", get(hello_world)) 
        .nest("/authService", auth_routers()).with_state(shared_state)
        .nest("/emailService", routers());

    let server = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(server, app).await.unwrap();
}

async fn hello_world(State(state): State<AppState>) -> &'static str {
    println!("scoped");
    "Hello World"
}

#[derive(Clone)]
struct AppState {
    db_pool: Pool<Postgres>,
}
