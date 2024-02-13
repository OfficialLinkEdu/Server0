pub mod auth_service {

    use argon2::{
        password_hash::{rand_core::OsRng, Salt, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier
    };
    use axum::{body, extract::{Json, State}};
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
                    .unwrap().hash.unwrap()
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
            use crate::endpoints::windows_ip;


                let _req = state
                    .http_client
                    .post(format!("http://{}:82/authService/createUser", windows_ip))
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await.unwrap();
       

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
        State(state): State<AppState>,        Json(payload): Json<LoginForm>
    ) -> Response {
        // First see if user exists
        println!("RUNNING SIGN IN");
        let result = sqlx::query_as::<_,PrivateUserInformation>("SELECT CAST(id AS text), password_hash, salt, user_name, school_code FROM users WHERE email = $1").bind(&payload.email).fetch_one(&state.db_pool).await;
        
        match result {
            Ok(query) => {

            
             let res =   Argon2::default().verify_password(payload.password.as_bytes(), &PasswordHash::new(&query.password_hash).unwrap());

  println!("{:?}",res);

                Response::builder().status(StatusCode::CONFLICT).body(body::Body::from("This account may not exist 1")).unwrap()

               
            }
            Err(e) =>
            {
                println!("UH OH: {:?}",e);
               // User dosen't exist 
               Response::builder().status(StatusCode::CONFLICT).body(body::Body::from("This account may not exist")).unwrap()

            }
        }

    }

    pub fn auth_routers() -> Router<AppState> {
        Router::new()
        .route("/registerUser", post(register_user))
        .route("/loginUser", post(sign_in_user))
    }
}
