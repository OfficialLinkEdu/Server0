pub mod auth_service {
    use std::os::macos::raw::stat;
    use std::sync::Arc;

    use axum::body::Body;
    use axum::extract::{Json, State};
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};

    use axum::routing::{get, post};
    use axum::Router;
    use rand::Rng;
    use serde::Deserialize;
    use sqlx::postgres::PgQueryResult;
    use sqlx::prelude::FromRow;
    use sqlx::query_as;

    use crate::AppState;
    #[derive(Debug, Deserialize, FromRow)]
    struct RegisterUserRequest {
        user_name: String,
        email: String,
        password: String,
        school_code: String,
    }

    //Used for registering a new user
    //#[debug_handler]
    pub async fn register_user(
        State(state): State<AppState>,
        Json(payload): Json<RegisterUserRequest>,
    ) -> StatusCode {
        //Step1: Query if an existing user exists (email)
        let result_query =
            sqlx::query_as::<_, RegisterUserRequest>("SELECT FROM users WHERE email = $1")
                .bind(&payload.email)
                .fetch_one(&state.db_pool)
                .await;

        match result_query {
            Ok(e) => {
                // Error: Tried registering with an account that already exists
                StatusCode::CONFLICT
            }
            Err(e) => {
                println!("ERROR IS {}\n", e);
                //Not found, register user
                //step 1: hash password
                let mut salt_array: [u8; 16] = [0; 16];
                rand::thread_rng().fill(&mut salt_array);
                println!("\nTHe salted password is {:?}\n", salt_array);
                let hashed_password =
                    bcrypt::hash_with_salt(payload.password, bcrypt::DEFAULT_COST, salt_array).unwrap();
                println!("\nThe Hashed password is {:?}", hashed_password);
                // Step 2: insert new user into users table
             let query_result =   sqlx::query("INSERT INTO users (email, password_hash, salt, user_name) VALUES($1, $2, $3, $4)").bind(payload.email).bind(hashed_password).bind(salt_array).bind(payload.user_name).execute(&state.db_pool).await;
             
             
                StatusCode::NOT_FOUND
            }
        }
    }

    pub fn auth_routers() -> Router<AppState> {
        Router::new().route("/registerUser", post(register_user))
    }
}
