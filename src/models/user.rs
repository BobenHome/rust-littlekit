use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UserQuery {
    pub name: Option<String>,
}
