use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PasswordEntry {
    pub id: Option<i32>,
    pub app_name: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

