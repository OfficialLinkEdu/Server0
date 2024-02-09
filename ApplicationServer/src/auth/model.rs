pub mod incoming_requests {
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

    #[derive(Debug, Deserialize, FromRow, Serialize)]
    pub struct RegisterUserRequest {
        pub email: String,
        pub password: String,
        pub school_code: String,
        pub user_name: String,
    }
}

pub mod database_models {
    use serde::Serialize;
    use sqlx::FromRow;

    #[derive(FromRow, Serialize)]
    pub struct UserPayLoadToSchool {
        pub id: String,
        pub user_name: String,
    }

    #[derive(sqlx::FromRow)]
    pub struct PrivateUserInformation {
        id: String,
       pub password_hash: String,
       pub salt: String,
        user_name: String,
        school_code: String,
    }
}

pub mod responses
{
    use serde::{Deserialize, Serialize};

    #[derive(Serialize,Deserialize,Debug)]
    pub struct UserResponseData
{
 pub  friends: Option<Vec<String>>,
 pub  id: String,
 pub  interests: Option<Vec<String>>,
 pub  public_id: String,
 pub  user_name: String,
 pub  jwt: String

}

}