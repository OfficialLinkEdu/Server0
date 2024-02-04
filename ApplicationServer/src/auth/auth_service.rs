pub mod auth_service {

    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        PasswordHasher,
    };
    use axum::extract::{Json, State};
    use axum::{http::StatusCode, response::Response};

    use crate::{
        auth::model::database_models::{PrivateUserInformation, UserPayLoadToSchool},
        auth::model::incoming_requests::RegisterUserRequest,
        AppState,
    };
    use axum::routing::post;
    use axum::Router;
    use serde::Deserialize;

    use crate::auth::model::responses::UserResponseData;
    //Used for registering a new user
    //#[debug_handler]
    async fn register_user(
        State(state): State<AppState>,
        Json(payload): Json<RegisterUserRequest>,
    ) -> Response {
        //Step1: Query if user already exists via unique email
        let result_query =
            sqlx::query_as::<_, PrivateUserInformation>("SELECT * FROM users WHERE email = $1")
                .bind(&payload.email)
                .fetch_one(&state.db_pool)
                .await;

        match result_query {
            // If queryable, then it exists
            Ok(_e) => Response::builder()
                .status(StatusCode::CONFLICT)
                .header("Content-Type", "application/json")
                .body(axum::body::Body::from(""))
                .unwrap(),
            Err(_e) => {
                //step 1: hash password
                let salt_array: SaltString = SaltString::generate(OsRng);
                let password_hash = argon2::Argon2::default()
                    .hash_password(&payload.password.as_bytes(), &salt_array)
                    .unwrap()
                    .hash
                    .unwrap()
                    .to_string();

                // Step 2: insert into central user database table
                sqlx::query("INSERT INTO users (email, password_hash, salt, user_name, school_code) VALUES($1, $2, $3, $4, $5)")
                .bind(&payload.email)
                .bind(password_hash)
                .bind(salt_array.to_string())
                .bind(&payload.user_name)
                .bind(&payload.school_code)
                .execute(&state.db_pool).await.unwrap();
                // on sucesfful register, creaete a request to the respective school url server to register users in

                // For future ref, just create the struct instead of query
                //If this is done use sqlx v4 feature to generate the uuid here isntead of DB
                let req: UserPayLoadToSchool = sqlx::query_as::<_, UserPayLoadToSchool>(
                    "SELECT id::text, user_name FROM users WHERE email = $1",
                )
                .bind(&payload.email)
                .fetch_one(&state.db_pool)
                .await
                .unwrap();

                let body = serde_json::to_string(&req).unwrap();
                println!("\n{body}\n");

                state
                    .http_client
                    .get("http://192.168.2.195:8080/")
                    .send()
                    .await
                    .unwrap();
                let _req = state
                    .http_client
                    .post("http://192.168.2.195:8080/authService/createUser")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await
                    .unwrap();

                // println!("----------\nThe req is:\n{:?}\n----------");
                let response_body = _req.json::<UserResponseData>().await.unwrap();
                let body = serde_json::to_string(&response_body).unwrap();
                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(axum::body::Body::from(body))
                    .unwrap()
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

        let _query_result: PrivateUserInformation = sqlx::query_as::<_, PrivateUserInformation>(
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
