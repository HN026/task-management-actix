use crate::db::db::{
    authenticate_user, create_task_db, create_user_db, delete_user_task_db, get_all_users_db,
    get_user_task_db, get_user_tasks_db, update_user_task_db,
};
use crate::jwt::jwt::generate_jwt;
use crate::model::models::{Info, SignInInput, TaskInput, TaskUpdate, UserInput, UserResponse};
use crate::model::state::AppState;
use actix_web::error::InternalError;
use actix_web::{http::StatusCode, web, Error, HttpResponse, Responder, ResponseError};
use log;
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    inner: sqlx::Error,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.inner)
    }
}

impl From<sqlx::Error> for CustomError {
    fn from(error: sqlx::Error) -> Self {
        CustomError { inner: error }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(format!("{}", self))
    }
}

pub async fn create_user(
    state: web::Data<AppState>,
    new_user: web::Json<UserInput>,
) -> Result<HttpResponse, Error> {
    log::info!(
        "Received request to create user with username {}",
        new_user.username,
    );

    let user = create_user_db(&state.pool, &new_user.into_inner())
        .await
        .map_err(|e| {
            log::error!("Failed to create user: {}", e);
            actix_web::Error::from(actix_web::error::InternalError::new(
                e,
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        })?;

    log::info!("Successfully created user with id {}", user.id);

    let token = generate_jwt(&user.id.to_string()).await.map_err(|e| {
        log::error!("Failed to generate JWT: {}", e);
        actix_web::Error::from(actix_web::error::InternalError::new(
            e,
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    })?;

    // Use the CreateUserResponse struct to construct the response
    let response = UserResponse { user, token };

    Ok(HttpResponse::Ok().json(response))
}

// Gets Users
pub async fn get_users(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    log::info!("Received request to get all users");

    let users = get_all_users_db(&state.pool).await.map_err(|e| {
        log::error!("Failed to get users: {}", e);
        InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    log::info!("Successfully fetched {} users", users.len());
    Ok(HttpResponse::Ok().json(users))
}

// Create Task
pub async fn create_task(
    new_task: web::Json<TaskInput>,
    user_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    log::info!(
        "Received request to create task for user with id {}",
        user_id
    );

    let task = create_task_db(&state.pool, &new_task.into_inner(), user_id.into_inner())
        .await
        .map_err(|e| {
            log::error!("Failed to create task: {}", e);
            CustomError::from(e)
        })?;

    log::info!("Successfully created task with id {}", task.id);
    Ok(HttpResponse::Ok().json(task))
}

pub async fn get_user_tasks(
    user_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    log::info!("Received request to get tasks for user with id {}", user_id);

    let tasks = get_user_tasks_db(&state.pool, user_id).await.map_err(|e| {
        log::error!("Failed to get tasks for user with id {}: {}", user_id, e);
        InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    log::info!(
        "Successfully fetched {} tasks for user with id {}",
        tasks.len(),
        user_id
    );

    Ok(HttpResponse::Ok().json(tasks))
}

pub async fn get_user_task(
    info: web::Path<Info>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let info = info.into_inner();

    let task = get_user_task_db(&state.pool, info.user_id, info.task_id)
        .await
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(HttpResponse::Ok().json(task))
}

pub async fn update_user_task(
    info: web::Path<Info>,
    state: web::Data<AppState>,
    task_update: web::Json<TaskUpdate>,
) -> Result<HttpResponse, Error> {
    let info = info.into_inner();
    log::info!(
        "Received request to update task with id {} for user with id {}",
        info.task_id,
        info.user_id
    );

    let task = update_user_task_db(&state.pool, info.clone(), task_update.into_inner())
        .await
        .map_err(|e| {
            log::error!(
                "Failed to update task with id {} for user with id {}: {}",
                info.task_id,
                info.user_id,
                e
            );
            InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    log::info!(
        "Successfully updated task with id {} for user with id {}",
        task.id,
        task.user_id
    );

    Ok(HttpResponse::Ok().json(task))
}

pub async fn delete_user_task(
    info: web::Path<Info>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let info = info.into_inner();
    log::info!(
        "Received request to delete task with id {} for user with id {}",
        info.task_id,
        info.user_id
    );

    let rows_affected = delete_user_task_db(&state.pool, info.clone())
        .await
        .map_err(|e| {
            log::error!(
                "Failed to delete task with id {} for user with id {}: {}",
                info.task_id,
                info.user_id,
                e
            );
            InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    if rows_affected == 0 {
        Err(InternalError::new("No task found to delete", StatusCode::NOT_FOUND).into())
    } else {
        log::info!(
            "Successfully deleted task with id {} for user with id {}",
            info.task_id,
            info.user_id
        );
        Ok(HttpResponse::NoContent().finish())
    }
}

pub async fn sign_in_handler(
    state: web::Data<AppState>,
    info: web::Json<SignInInput>,
) -> impl Responder {
    let username = &info.username;
    let password = &info.password;

    match authenticate_user(&state.pool, username, password).await {
        Ok(user) => match generate_jwt(&user.id.to_string()).await {
            Ok(token) => {
                let response = UserResponse { user, token };
                HttpResponse::Ok().json(response)
            }
            Err(e) => {
                log::error!("JWT generation failed: {}", e);
                HttpResponse::InternalServerError().body("Failed to generate token")
            }
        },
        Err(e) => {
            log::error!("Authentication failed: {}", e);
            if e.to_string().contains("Invalid username or password") {
                HttpResponse::Unauthorized().body("Invalid credentials")
            } else {
                HttpResponse::InternalServerError().body("Internal server error")
            }
        }
    }
}
