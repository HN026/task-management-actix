use crate::db::migrations::run_migrations;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn create_pool_and_run_migrations() -> Result<sqlx::PgPool, std::io::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to the database successfully");
            match run_migrations(&pool).await {
                Ok(_) => println!("Migrations ran successfully"),
                Err(e) => {
                    println!("Failed to run migrations: {}", e);
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                }
            }
            pool
        }
        Err(e) => {
            println!("Failed to connect to the database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    Ok(pool)
}
