pub mod incoming_requests {
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

    // Used for registering user
    #[derive(Debug, Deserialize, FromRow, Serialize)]
    pub struct RegisterUserRequest {
        pub email: String,
        pub password: String,
        pub school_code: String,
        pub user_name: String,
        pub school_email: String
    }

    // Used for signing in
    #[derive(Debug, Deserialize)]
    pub struct LoginForm {
      pub  email: String,
      pub  password: String,
    }
}

pub mod database_models {
    use serde::Serialize;
    use sqlx::FromRow;


    // ! UserPayLoadToSchool is the only value that can be sent to fetch school account
    #[derive(FromRow, Serialize,Clone)]
    pub struct UserPayLoadToSchool {
        pub id: String,
        pub user_name: String,

        pub school_email: String
    }


    #[derive(sqlx::FromRow)]
    pub struct PrivateUserInformation {
      pub  id: String,
        pub password_hash: String,
        pub salt: String,
      email: String,
        school_code: String,
    }
}

pub mod responses {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserResponseData {
        pub friends: Option<Vec<String>>,
        pub id: String,
        pub interests: Option<Vec<String>>,
        pub public_id: String,
        pub user_name: String,
        pub jwt: String,
        pub profile_picture_url: Option<String>,
        pub gender: Option<i16>
    }

}
