mod db;
mod handlers;
mod jwt;
mod model;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{http::header, web, App, HttpServer, Responder};
use db::server::create_pool_and_run_migrations;
use dotenv::dotenv;
use handlers::handlers::{
    create_task, create_user, delete_user_task, get_user_task, get_user_tasks, get_users,
    sign_in_handler, update_user_task,
};
use model::state::AppState;

async fn index() -> impl Responder {
    format!("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let pool = create_pool_and_run_migrations().await?;

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
            .route("/sign_in", web::post().to(sign_in_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
