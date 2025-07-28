use crate::app::config::Config;
use sqlx::{Pool, Postgres, postgres::PgPool};
use std::time::Duration;
use tokio::time::sleep;

pub async fn init_primary_db(config: &Config) -> Result<Pool<Postgres>, sqlx::Error> {
    println!("{:?}", config);
    let db_config = config
        .database
        .as_ref()
        .expect("Database configuration is required");

    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_config.username, db_config.password, db_config.host, db_config.port, db_config.database
    );

    let pool = connect_with_retries(&database_url, db_config.retry).await;
    pool
}

pub async fn connect_with_retries(
    database_url: &str,
    max_retries: u16,
) -> Result<PgPool, sqlx::Error> {
    let mut attempts = 0;

    loop {
        match PgPool::connect(database_url).await {
            Ok(pool) => {
                println!("Successfully connected to the database.");
                return Ok(pool);
            }
            Err(e) => {
                attempts += 1;
                if attempts >= max_retries {
                    println!(
                        "Failed to connect to the database after {} attempts.",
                        attempts
                    );
                    return Err(e);
                }
                println!(
                    "Failed to connect to the database. Attempt {}/{}. Retrying in 2 seconds...",
                    attempts, max_retries
                );
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
