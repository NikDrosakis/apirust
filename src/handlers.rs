use actix_web::{get, web, Responder};
use sqlx::MySqlPool;
use crate::models::{User, Post};

#[get("/user")]
async fn get_user(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as::<_, User>("SELECT * FROM user")
        .fetch_all(&**db_pool)
        .await;

    match result {
        Ok(users) => web::Json(users),
        Err(_e) => web::Json(vec![User {
            id: 0,
            name: "".to_string(),
            mail: "".to_string(),
            firstname: "".to_string(),
            lastname: "".to_string(),
        }]),  // Create a Vec<User> to match the expected type
    }
}

#[get("/post")]
async fn get_post(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Post>("SELECT * FROM post")
        .fetch_all(&**db_pool)
        .await;

    match result {
        Ok(posts) => web::Json(posts),
        Err(_e) => web::Json(vec![Post {
            id: 0,
            uid: 1,
            title: "".to_string(),
            content: Some("".to_string()),
        }]),  // Create a Vec<User> to match the expected type
    }
}