use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
}
