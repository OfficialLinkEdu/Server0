pub mod auth_service {
    use std::{os::macos::raw::stat, sync::Arc};

    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        PasswordHasher,
    };
    use axum::http::StatusCode;
    use axum::{
        extract::{Json, State},
        http::request,
    };

    use crate::AppState;
    use axum::routing::{get, post};
    use axum::Router;
    use serde::Deserialize;
    use sqlx::prelude::FromRow;
    #[derive(Debug, Deserialize, FromRow)]
    struct RegisterUserRequest {
        user_name: String,
        email: String,
        password: String,
        school_code: String,
    }

    #[derive(sqlx::FromRow)]
    struct PrivateUserInformation {
        id: String,
        password_hash: String,
        salt: String,
        user_name: String,
    }

    //Used for registering a new user
    //#[debug_handler]
    async fn register_user(
        State(state): State<AppState>,
        Json(payload): Json<RegisterUserRequest>,
    ) -> StatusCode {
        //Step1: Query if an existing user exists (email)
        let result_query = sqlx::query_as::<_, PrivateUserInformation>(
            "SELECT CAST(id AS Text), password_hash, salt, user_name FROM users WHERE email = $1",
        )
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
                let mut salt_array: SaltString = SaltString::generate(OsRng);
                println!("Salt array is: {}", salt_array);
                let password_hash = argon2::Argon2::default()
                    .hash_password(&payload.password.as_bytes(), &salt_array)
                    .unwrap()
                    .hash
                    .unwrap()
                    .to_string();
                println!("\nHash is: {:?}", password_hash);

                // Step 2: insert new user into users table
                let query_result =   sqlx::query("INSERT INTO users (email, password_hash, salt, user_name) VALUES($1, $2, $3, $4)").bind(&payload.email).bind(password_hash).bind(salt_array.to_string()).bind(payload.user_name).execute(&state.db_pool).await;
                // on sucesfful register, creaete a request to the respective school url server to register users in
                let res = state
                    .http_client
                    .get("http://localhost:85/")
                    .send()
                    .await
                    .unwrap();
                println!("{:?}", res);

                match query_result {
                    Ok(result) => {
                        let new_payload: LoginForm = LoginForm {
                            email: payload.email,
                            password: payload.password,
                        };
                        return sign_in_user(Json(new_payload), State(state)).await;
                    }
                    Err(_) => {
                        return StatusCode::UNPROCESSABLE_ENTITY;
                    }
                }
            }
        }
    }

    #[derive(Debug, Deserialize)]

    struct LoginForm {
        email: String,
        password: String,
    }

    async fn sign_in_user(
        Json(payload): Json<LoginForm>,
        State(state): State<AppState>,
    ) -> StatusCode {
        /*
        Since this will be the only sign in,
        re-query user information
            */

        let query_result: PrivateUserInformation = sqlx::query_as::<_, PrivateUserInformation>(
            "SELECT CAST(id AS TEXT), password_hash, salt, user_name FROM users WHERE email = $1",
        )
        .bind(&payload.email)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

        state.http_client.post("192.168.2.195")

        StatusCode::OK
    }

    pub fn auth_routers() -> Router<AppState> {
        Router::new().route("/registerUser", post(register_user))
    }
}
