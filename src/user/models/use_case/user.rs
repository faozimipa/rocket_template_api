use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredential {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub data: GetUserResponse,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllUserResponse {
    pub data: Vec<GetUserResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub plain_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: String,
}
