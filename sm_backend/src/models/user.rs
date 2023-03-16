use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub hashed_password: String,
    pub is_admin: bool
}
