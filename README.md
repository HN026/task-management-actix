# User Task Management API

This project is a RESTful API built with Rust and Actix-web. It provides a simple task management system where users can create, read, update, and delete tasks.

## Endpoints

Here are the available endpoints:

- `GET /`: Returns a welcome message.
- `POST /users`: Creates a new user.
- `GET /get_users`: Retrieves all users.
- `POST /users/{user_id}/tasks`: Creates a new task for a specific user.
- `GET /users/{user_id}/tasks`: Retrieves all tasks for a specific user.
- `GET /users/{user_id}/tasks/{task_id}`: Retrieves a specific task for a specific user.
- `PUT /users/{user_id}/tasks/{task_id}`: Updates a specific task for a specific user.
- `DELETE /users/{user_id}/tasks/{task_id}`: Deletes a specific task for a specific user.

## Requirements

To run this project, you need to have Rust installed on your machine. You also need a SqlLite database, as this project uses SQLx for database operations.

## Prerequisites

Before running this project, you need to set the following environment variables:

- `DATABASE_URL`: The URL to your database, which the application will use to store and manage data.
- `SECRET_KEY`: A secret key used for securing the application, such as for signing JWT tokens.

## Running the Project

1. Clone the repository.
2. Set up your database and update the database URL in the [`.env`] file.
3. Run `cargo build` to build the project.
4. Run `cargo run` to start the server.
5. To run handler test, Run `cargo test` to start the tests.

## Testing

You can use any HTTP client like curl or Postman to test the API. Make sure to replace `{user_id}` and `{task_id}` with actual IDs in the endpoints.