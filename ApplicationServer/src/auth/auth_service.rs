pub mod auth_service {
    use std::sync::Arc;

    use axum::body::Body;
    use axum::extract::{Json, State};
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};

    use axum::routing::{get, post};
    use axum::Router;
    use serde::Deserialize;
    use sqlx::postgres::PgQueryResult;
    use sqlx::prelude::FromRow;
    use sqlx::query_as;

    use crate::AppState;
    #[derive(Deserialize,FromRow)]
    struct RegisterUserRequest {
        user_name: String,
        email: String,
        password: String,
        school_code: String,
    }

    //#[debug_handler]
    //Used for registering a new user
    pub async fn register_user(
        State(state): State<AppState>,
        Json(payload): Json<RegisterUserRequest>,
    ) -> String {
        //Step1: Query if an existing user exists (email)
       let result_query = sqlx::query_as::<_,RegisterUserRequest>("SELECT FROM users WHERE email = $1").bind(&payload.email).fetch_one(&state.db_pool).await.unwrap_or_else(|Err|{});
   
        let query = sqlx::query(
            format!(
                "INSERT INTO users (email, password_hash, user_name) VALUES ('{}','{}','{}')",
                payload.email, payload.password, payload.user_name
            )
            .as_str(),
        )
        //.bind(value)
        .execute(&state.db_pool)
        .await
        .unwrap();
        println!("DONE");

        format!("Hi from")
    }

    pub fn auth_routers() -> Router<AppState> {
        Router::new().route("/registerUser", post(register_user))
    }
}
