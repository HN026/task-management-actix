mod lib {
    pub mod handlers;
    pub mod models;
    pub mod state;
}

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{http::header, web, App, HttpServer, Responder};
use dotenv::dotenv;
use lib::handlers::{
    create_task, create_user, delete_user_task, get_user_task, get_user_tasks, get_users,
    update_user_task,
};
use lib::state::AppState;
use sqlx::postgres::PgPoolOptions;
use std::env;

async fn index() -> impl Responder {
    format!("Hello, world!")
}

async fn run_migrations(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
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

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(Data::new(AppState { pool: pool.clone() }))
            .wrap(cors)
            .route("/", web::get().to(index))
            .route("/users", web::post().to(create_user))
            .route("/get_users", web::get().to(get_users))
            .route("/users/{user_id}/tasks", web::post().to(create_task))
            .route("/users/{user_id}/tasks", web::get().to(get_user_tasks))
            .route(
                "/users/{user_id}/tasks/{task_id}",
                web::get().to(get_user_task),
            )
            .route(
                "/users/{user_id}/tasks/{task_id}",
                web::put().to(update_user_task),
            )
            .route(
                "/users/{user_id}/tasks/{task_id}",
                web::delete().to(delete_user_task),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
