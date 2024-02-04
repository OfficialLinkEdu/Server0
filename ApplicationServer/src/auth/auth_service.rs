pub mod auth_service {

    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        PasswordHasher,
    };
    use axum::http::StatusCode;
    use axum::{
        extract::{Json, State},
        http::request,
    };
    use serde_json::json;

    use crate::{
        auth::model::database_models::{PrivateUserInformation, UserPayLoad},
        auth::model::incoming_requests::RegisterUserRequest,
        AppState,
    };
    use axum::routing::{get, post};
    use axum::Router;
    use serde::{Deserialize, Serialize};
    use sqlx::sqlx_macros::FromRow;

    //Used for registering a new user
    //#[debug_handler]
    async fn register_user(
        State(state): State<AppState>,
        Json(payload): Json<RegisterUserRequest>,
    ) -> StatusCode {
        //Step1: Query if an existing user exists (email)
        let result_query =
            sqlx::query_as::<_, PrivateUserInformation>("SELECT * FROM users WHERE email = $1")
                .bind(&payload.email)
                .fetch_one(&state.db_pool)
                .await;

        match result_query {
            Ok(e) => {
                // Error: Tried registering with an account that already exists
                StatusCode::CONFLICT
            }
            Err(e) => {
                //Not found, register user
                //step 1: hash password
                let mut salt_array: SaltString = SaltString::generate(OsRng);
                let password_hash = argon2::Argon2::default()
                    .hash_password(&payload.password.as_bytes(), &salt_array)
                    .unwrap()
                    .hash
                    .unwrap()
                    .to_string();

                // Step 2: insert new user into users table
                sqlx::query("INSERT INTO users (email, password_hash, salt, user_name) VALUES($1, $2, $3, $4)")
                .bind(&payload.email)
                .bind(password_hash)
                .bind(salt_array.to_string())
                .bind(&payload.user_name)
                .execute(&state.db_pool).await.unwrap();
                // on sucesfful register, creaete a request to the respective school url server to register users in

                let req: UserPayLoad = sqlx::query_as::<_, UserPayLoad>(
                    "SELECT id::text, user_name FROM users WHERE email = $1",
                )
                .bind(&payload.email)
                .fetch_one(&state.db_pool)
                .await
                .unwrap();
                let body = serde_json::to_string(&req).unwrap();
                let req = state
                    .http_client
                    .post("http://192.168.2.195:8080/authService/createUser")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await
                    .unwrap();

                StatusCode::UNPROCESSABLE_ENTITY
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

        StatusCode::OK
    }

    pub fn auth_routers() -> Router<AppState> {
        Router::new().route("/registerUser", post(register_user))
    }
}
