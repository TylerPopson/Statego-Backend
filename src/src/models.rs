use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserDetails {
    pub email: String,
    pub username: String,
    pub pass: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize)]
pub struct UserData{
    pub id: i32,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponseData{
    pub user_data: Vec<UserData>
}