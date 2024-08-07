use crate::model::models::{Info, Task, TaskInput, TaskUpdate, User, UserInput};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use sqlx::{Error, PgPool};

pub async fn create_user_db(pool: &PgPool, new_user: &UserInput) -> Result<User, Error> {
    let password_hash = hash(&new_user.password, DEFAULT_COST)
        .map_err(|e| Error::protocol(format!("Bcrypt error: {}", e)))?;

    let record = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash, email)
        VALUES ($1, $2, $3)
        RETURNING id, username, password_hash, email
        "#,
        &new_user.username,
        &password_hash,
        &new_user.email,
    )
    .fetch_one(pool)
    .await?;

    println!("Running this function");

    Ok(User {
        id: record.id,
        username: record.username,
        password_hash: record.password_hash,
        email: record.email,
    })
}

pub async fn get_all_users_db(pool: &PgPool) -> Result<Vec<User>, Error> {
    let records = sqlx::query!(
        r#"
        SELECT id, username, password_hash, email FROM users
        "#
    )
    .fetch_all(pool)
    .await?;

    let users: Vec<User> = records
        .into_iter()
        .map(|record| User {
            id: record.id,
            username: record.username,
            password_hash: record.password_hash,
            email: record.email,
        })
        .collect();

    Ok(users)
}

pub async fn create_task_db(
    pool: &PgPool,
    new_task: &TaskInput,
    user_id: i32,
) -> Result<Task, Error> {
    let record = sqlx::query!(
        r#"
        INSERT INTO tasks (title, description, due_date, status, user_id) VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, description, due_date, status, user_id
        "#,
        &new_task.title,
        &new_task.description,
        new_task.due_date.unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0)),
        &new_task.status,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(Task {
        id: record.id,
        title: record.title,
        description: record.description.expect("Description is missing"),
        due_date: record.due_date,
        status: record.status,
        user_id: record.user_id.expect("User ID is missing"),
    })
}

pub async fn get_user_tasks_db(pool: &PgPool, user_id: i32) -> Result<Vec<Task>, Error> {
    let records = sqlx::query!(
        r#"
        SELECT id, title, description, due_date, status, user_id FROM tasks WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_all(pool)
    .await?;

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

    Ok(tasks)
}

pub async fn get_user_task_db(pool: &PgPool, user_id: i32, task_id: i32) -> Result<Task, Error> {
    let record = sqlx::query!(
        r#"
        SELECT id, title, description, due_date, status, user_id FROM tasks WHERE user_id = $1 AND id = $2
        "#,
        user_id, task_id
    )
    .fetch_one(pool)
    .await?;

    Ok(Task {
        id: record.id,
        title: record.title,
        description: record.description.expect("Description is missing"),
        due_date: record.due_date,
        status: record.status,
        user_id: record.user_id.expect("User ID is missing"),
    })
}

pub async fn update_user_task_db(
    pool: &PgPool,
    info: Info,
    task_update: TaskUpdate,
) -> Result<Task, Error> {
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
    .await?;

    Ok(Task {
        id: record.id,
        title: record.title,
        description: record.description.expect("Description is missing"),
        due_date: record.due_date,
        status: record.status,
        user_id: record.user_id.expect("User ID is missing"),
    })
}

pub async fn delete_user_task_db(pool: &PgPool, info: Info) -> Result<u64, Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM tasks WHERE user_id = $1 AND id = $2
        "#,
        info.user_id,
        info.task_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn authenticate_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<User, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, email
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await?;

    if verify(password, &user.password_hash)
        .map_err(|_| Error::protocol("Password verification failed"))?
    {
        Ok(User {
            id: user.id,
            username: user.username,
            password_hash: String::new(),
            email: user.email,
        })
    } else {
        Err(Error::protocol("Invalid username or password"))
    }
}
