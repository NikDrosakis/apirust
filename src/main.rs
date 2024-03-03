use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod handlers;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::init_pool().await.expect("Failed to initialize pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api")
                .service(handlers::get_user)
                .service(handlers::get_post))
    })
        .bind("localhost:8090")?
        .run()
        .await
}