use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    pub name: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskInput {
    pub title: String,
    pub description: String,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub user_id: i32,
    pub task_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskUpdate {
    pub title: String,
    pub description: String,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub status: String,
}
