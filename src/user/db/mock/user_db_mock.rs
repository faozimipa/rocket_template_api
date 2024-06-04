use crate::user::{
    errors::CustomError,
    models::{
        use_case::user::{ GetAllUserResponse, GetUserResponse, UserCredential, UserProfile },
        user::User,
    },
    repository::UserDbTrait,
};

pub struct MockUserDB {}

#[async_trait]
impl UserDbTrait for MockUserDB {
    async fn login(&self, credential: UserCredential) -> Result<UserProfile, CustomError> {
        Ok(UserProfile {
            data: GetUserResponse {
                id: "1234abcd".into(),
                name: format!("s name"),
                email: format!("{}@example.com", credential.email),
            },
            token: "1234abcd".into(),
        })
    }
    async fn get_all(&self) -> Result<GetAllUserResponse, CustomError> {
        Ok(GetAllUserResponse {
            data: Vec::new(),
        })
    }

    async fn get_by_id(&self, id: &str) -> Result<GetUserResponse, CustomError> {
        Ok(GetUserResponse {
            id: id.to_owned(),
            name: format!("{}'s name", id),
            email: format!("{}@example.com", id),
        })
    }

    async fn create(&self, _user: User) -> Result<String, CustomError> {
        Ok("1234abcd".into())
    }

    async fn delete(&self, _id: &str) -> Result<(), CustomError> {
        Ok(())
    }
}
