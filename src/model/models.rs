use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserInput {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub status: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TaskInput {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub due_date: Option<chrono::NaiveDateTime>,
    #[validate(length(min = 1))]
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Info {
    pub user_id: i32,
    pub task_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TaskUpdate {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub due_date: Option<chrono::NaiveDateTime>,
    #[validate(length(min = 1))]
    pub status: String,
}

#[derive(Deserialize)]
pub struct SignInInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user: User,
    pub token: String,
}
