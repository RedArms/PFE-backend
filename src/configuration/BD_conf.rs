use std::env;
use dotenv::dotenv;
use sqlx::PgPool;
use sqlx_core::Error;

pub async fn init_db_pool() -> Result<PgPool, Error> {
    dotenv().ok();

    // Retrieve the database URL from the environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");

    // Connect to the database
    PgPool::connect(&database_url).await
}