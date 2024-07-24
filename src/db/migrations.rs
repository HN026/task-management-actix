use sqlx::Error;
use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        username TEXT NOT NULL UNIQUE,
        password_hash TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            due_date TIMESTAMP,
            status TEXT NOT NULL,
            user_id INTEGER REFERENCES users(id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::test;
    use dotenv::dotenv;
    use sqlx::Error;
    use sqlx::PgPool;
    use std::env;

    #[test]
    async fn test_run_migrations() -> Result<(), Error> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPool::connect(&database_url).await?;

        run_migrations(&pool).await?;
        let row: (bool,) =
            sqlx::query_as("SELECT EXISTS (SELECT FROM pg_tables WHERE tablename = 'users')")
                .fetch_one(&pool)
                .await?;
        assert!(row.0, "users table does not exist");
        let row: (bool,) =
            sqlx::query_as("SELECT EXISTS (SELECT FROM pg_tables WHERE tablename = 'tasks')")
                .fetch_one(&pool)
                .await?;
        assert!(row.0, "tasks table does not exist");

        Ok(())
    }
}
