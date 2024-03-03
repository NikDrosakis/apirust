use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub async fn init_pool() -> Result<MySqlPool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
