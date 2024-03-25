use axum::Router;

use crate::{api::controllers::{auth_controller::auth_service::auth_routers, email_controller::email_service::email_routers}, hello_world, AppState};



pub fn all_routes() -> Router<AppState>
{
    axum::Router::new()
    .nest("/authService", auth_routers())
    .nest("/emailService", email_routers())
}