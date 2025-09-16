use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn init_db(db_url: &str) -> Db {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Failed to connect to Postgres")
} 
