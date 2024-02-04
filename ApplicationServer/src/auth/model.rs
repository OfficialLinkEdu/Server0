pub mod incoming_requests {
    use serde::{Deserialize,Serialize};
    use sqlx::FromRow;

    #[derive(Debug, Deserialize, FromRow, Serialize)]
    pub struct RegisterUserRequest {
     pub   email: String,
       pub password: String,
        school_code: String,
  pub      user_name: String,
    }
}

pub mod database_models {
    use sqlx::FromRow;
    use serde::Serialize;

    #[derive(FromRow, Serialize)]
    pub struct UserPayLoad {
        pub id: String,
        pub user_name: String,
    }

    #[derive(sqlx::FromRow)]
pub    struct PrivateUserInformation {
        id: String,
        password_hash: String,
        salt: String,
        user_name: String,
    }
}
