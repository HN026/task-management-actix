use crate::handlers::handlers::{
    create_task, create_user, get_user_task, get_user_tasks, get_users, update_user_task,
};
use crate::model::models::{Task, TaskInput, TaskUpdate, User, UserInput};
use crate::model::state::AppState;
use actix_web::{http::StatusCode, test, web, App};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::env;

// Create User Test
#[actix_rt::test]
async fn test_create_user() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::<Postgres>::connect(&database_url).await.unwrap();

    let data = AppState { pool };

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(data))
            .route("/create_user", web::post().to(create_user)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/create_user")
        .set_json(&UserInput {
            name: "Huzaifa".into(),
        })
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let user: User = test::read_body_json(resp).await;
    assert_eq!(user.name, "Huzaifa");
}

// Get Users Test
#[actix_rt::test]
async fn test_get_users() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::<Postgres>::connect(&database_url).await.unwrap();

    let data = AppState { pool };

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(data))
            .route("/get_users", web::get().to(get_users)),
    )
    .await;

    let req = test::TestRequest::get().uri("/get_users").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let users: Vec<User> = test::read_body_json(resp).await;

    assert!(!users.is_empty());
}

// Create Task Test
#[actix_rt::test]
async fn test_create_task() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::<Postgres>::connect(&database_url).await.unwrap();

    let data = AppState { pool };

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(data))
            .route("/users/{user_id}/tasks", web::post().to(create_task)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/users/2/tasks")
        .set_json(&TaskInput {
            title: "Test task".into(),
            description: "Test description".into(),
            due_date: None,
            status: "todo".into(),
        })
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let task: Task = test::read_body_json(resp).await;
    assert_eq!(task.title, "Test task");
    assert_eq!(task.description, "Test description");
    assert_eq!(task.status, "todo");
}

#[actix_rt::test]
async fn test_get_user_tasks() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::<Postgres>::connect(&database_url).await.unwrap();

    let data = AppState { pool };

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(data))
            .route("/users/{user_id}/tasks", web::get().to(get_user_tasks)),
    )
    .await;

    let req = test::TestRequest::get().uri("/users/2/tasks").to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let tasks: Vec<Task> = test::read_body_json(resp).await;
    assert!(!tasks.is_empty(), "Should return at least one task");
}

#[actix_rt::test]
async fn test_get_user_task() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::<Postgres>::connect(&database_url).await.unwrap();

    let data = AppState { pool };

    let mut app = test::init_service(App::new().app_data(web::Data::new(data)).route(
        "/users/{user_id}/tasks/{task_id}",
        web::get().to(get_user_task),
    ))
    .await;

    let req = test::TestRequest::get()
        .uri("/users/2/tasks/2")
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let task: Task = test::read_body_json(resp).await;
    assert_eq!(task.user_id, 2, "Should return the task for user 2");
    assert_eq!(task.id, 1, "Should return the task with id 1");
}

#[actix_rt::test]
async fn test_update_user_task() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::<Postgres>::connect(&database_url).await.unwrap();

    let data = AppState { pool };

    let mut app = test::init_service(App::new().app_data(web::Data::new(data)).route(
        "/users/{user_id}/tasks/{task_id}",
        web::put().to(update_user_task),
    ))
    .await;

    let task_update = TaskUpdate {
        title: "Updated Title".into(),
        description: "Updated Description".into(),
        due_date: None,
        status: "completed".into(),
    };

    let req = test::TestRequest::put()
        .uri("/users/2/tasks/2")
        .set_json(&task_update)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let task: Task = test::read_body_json(resp).await;
    assert_eq!(task.user_id, 2, "Should return the task for user 2");
    assert_eq!(task.id, 1, "Should return the task with id 1");
    assert_eq!(task.title, "Updated Title", "Title should be updated");
    assert_eq!(
        task.description, "Updated Description",
        "Description should be updated"
    );
    assert_eq!(task.status, "completed", "Status should be updated");
}

// #[actix_rt::test]
// async fn test_delete_user_task() {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

//     let pool = Pool::<Postgres>::connect(&database_url).await.unwrap();

//     let data = AppState { pool };

//     let mut app = test::init_service(App::new().app_data(web::Data::new(data)).route(
//         "/users/{user_id}/tasks/{task_id}",
//         web::delete().to(delete_user_task),
//     ))
//     .await;

//     let req = test::TestRequest::delete()
//         .uri("/users/2/tasks/1")
//         .to_request();

//     let resp = test::call_service(&mut app, req).await;

//     assert_eq!(resp.status(), StatusCode::NO_CONTENT);
// }
