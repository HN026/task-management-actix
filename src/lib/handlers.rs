use crate::lib::models::{Info, Task, TaskInput, TaskUpdate, User, UserInput};
use crate::lib::state::AppState;
use actix_web::error::InternalError;
use actix_web::{http::StatusCode, web, Error, HttpResponse, ResponseError};
use chrono::NaiveDateTime;
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

// Creates User
pub async fn create_user(
    state: web::Data<AppState>,
    new_user: web::Json<UserInput>,
) -> Result<HttpResponse, Error> {
    log::info!(
        "Received request to create user with name {}",
        new_user.name
    );

    let pool = &state.pool;
    let record = sqlx::query!(
        r#"
        INSERT INTO users (name) VALUES ($1)
        RETURNING id, name
        "#,
        &new_user.name,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        log::error!("Failed to create user: {}", e);
        InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    let user = User {
        id: record.id,
        name: record.name,
    };

    log::info!("Successfully created user with id {}", user.id);

    Ok(HttpResponse::Ok().json(user))
}

// Gets Users
pub async fn get_users(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    log::info!("Received request to get all users");

    let pool = &state.pool;
    let records = sqlx::query!(
        r#"
        SELECT id, name FROM users
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        log::error!("Failed to get users: {}", e);
        InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    let users: Vec<User> = records
        .into_iter()
        .map(|record| User {
            id: record.id,
            name: record.name,
        })
        .collect();

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

    let pool = &state.pool;

    let record = sqlx::query!(
        r#"
        INSERT INTO tasks (title, description, due_date, status, user_id) VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, description, due_date, status, user_id
        "#,
        &new_task.title,
        &new_task.description,
        &new_task.due_date.unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0)),
        &new_task.status,
        &user_id.into_inner(),
    ).fetch_one(pool).await.map_err(|e| {
        log::error!("Failed to create task: {}", e);
        CustomError::from(e)
    })?;

    let task = Task {
        id: record.id,
        title: record.title,
        description: record.description.expect("Description is missing"),
        due_date: record.due_date,
        status: record.status,
        user_id: record.user_id.expect("User ID is missing"),
    };

    log::info!("Successfully created task with id {}", task.id);

    Ok(HttpResponse::Ok().json(task))
}

pub async fn get_user_tasks(
    user_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    log::info!("Received request to get tasks for user with id {}", user_id);

    let pool = &state.pool;

    let records = sqlx::query!(
        r#"
        SELECT id, title, description, due_date, status, user_id FROM tasks WHERE user_id = $1
        "#,
        &user_id,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        log::error!("Failed to get tasks for user with id {}: {}", user_id, e);
        InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    let tasks: Vec<Task> = records
        .into_iter()
        .map(|record| Task {
            id: record.id,
            title: record.title,
            description: record.description.expect("Description is missing"),
            due_date: record.due_date,
            status: record.status,
            user_id: record.user_id.expect("User ID is missing"),
        })
        .collect();

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
    let pool = &state.pool;

    let record = sqlx::query!(
        r#"
        SELECT id, title, description, due_date, status, user_id FROM tasks WHERE user_id = $1 AND id = $2
        "#,
        info.user_id, info.task_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    let task = Task {
        id: record.id,
        title: record.title,
        description: record.description.expect("Description is missing"),
        due_date: record.due_date,
        status: record.status,
        user_id: record.user_id.expect("User ID is missing"),
    };

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

    let pool = &state.pool;

    let record = sqlx::query!(
        r#"
        UPDATE tasks
        SET title = $1, description = $2, due_date = $3, status = $4
        WHERE id = $5 AND user_id = $6
        RETURNING id, title, description, due_date, status, user_id
        "#,
        task_update.title,
        task_update.description,
        task_update.due_date,
        task_update.status,
        info.task_id,
        info.user_id,
    )
    .fetch_one(pool)
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

    let task = Task {
        id: record.id,
        title: record.title,
        description: record.description.expect("Description is missing"),
        due_date: record.due_date,
        status: record.status,
        user_id: record.user_id.expect("User ID is missing"),
    };

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

    let pool = &state.pool;

    let deleted = sqlx::query!(
        r#"
        DELETE FROM tasks WHERE user_id = $1 AND id = $2
        "#,
        info.user_id,
        info.task_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        log::error!(
            "Failed to delete task with id {} for user with id {}: {}",
            info.task_id,
            info.user_id,
            e
        );
        actix_web::error::InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    if deleted.rows_affected() == 0 {
        Err(
            actix_web::error::InternalError::new("No task found to delete", StatusCode::NOT_FOUND)
                .into(),
        )
    } else {
        log::info!(
            "Successfully deleted task with id {} for user with id {}",
            info.task_id,
            info.user_id
        );
        Ok(HttpResponse::NoContent().finish())
    }
}

#[cfg(test)]
mod test;
