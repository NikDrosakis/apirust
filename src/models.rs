use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub mail: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: u32,
    pub uid: u32,
    pub title: String,
    pub content: Option<String>,
}