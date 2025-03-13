use axum::{Router, routing::{get, post, put, delete}, Extension, extract::{State, Path, Json}, http::StatusCode};
use sqlx::{PgPool, FromRow};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, FromRow, Serialize)]
struct User {
    id: i32,
    username: String,
    role_id: i32,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Role {
    id: i32,
    name: String,
    parent_id: Option<i32>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Permission {
    id: i32,
    action: String,
    role_id: i32,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct Task {
    id: i32,
    title: String,
    description: String,
    owner_id: i32,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
    role_id: i32,
}

#[derive(Debug, Deserialize)]
struct CreateTask {
    title: String,
    description: String,
    owner_id: i32,
}

async fn check_permission(pool: &PgPool, role_id: i32, action: &str) -> bool {
    let result = sqlx::query!(
        "WITH RECURSIVE role_hierarchy AS (
            SELECT id, parent_id FROM roles WHERE id = $1
            UNION ALL
            SELECT r.id, r.parent_id FROM roles r
            INNER JOIN role_hierarchy rh ON r.id = rh.parent_id
        )
        SELECT 1 FROM permissions p
        INNER JOIN role_hierarchy rh ON p.role_id = rh.id
        WHERE p.action = $2 LIMIT 1",
        role_id,
        action
    )
    .fetch_optional(pool)
    .await
    .unwrap();
    
    result.is_some()
}

async fn create_task(
    State(pool): State<Arc<PgPool>>, 
    Json(payload): Json<CreateTask>
) -> Result<Json<Task>, StatusCode> {
    if !check_permission(&pool, payload.owner_id, "create_task").await {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let task = sqlx::query_as!(
        Task,
        "INSERT INTO tasks (title, description, owner_id) VALUES ($1, $2, $3) RETURNING id, title, description, owner_id",
        payload.title,
        payload.description,
        payload.owner_id
    )
    .fetch_one(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(task))
}

async fn update_task(
    State(pool): State<Arc<PgPool>>, 
    Path(task_id): Path<i32>,
    Json(payload): Json<CreateTask>
) -> Result<Json<Task>, StatusCode> {
    if !check_permission(&pool, payload.owner_id, "edit_task").await {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let task = sqlx::query_as!(
        Task,
        "UPDATE tasks SET title = $1, description = $2 WHERE id = $3 RETURNING id, title, description, owner_id",
        payload.title,
        payload.description,
        task_id
    )
    .fetch_one(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(task))
}

async fn delete_task(
    State(pool): State<Arc<PgPool>>, 
    Path(task_id): Path<i32>,
    Path(user_id): Path<i32>
) -> Result<StatusCode, StatusCode> {
    if !check_permission(&pool, user_id, "delete_task").await {
        return Err(StatusCode::FORBIDDEN);
    }
    
    sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id)
        .execute(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://user:password@localhost/db_name")
        .await
        .expect("Failed to connect to database");
    
    let app = Router::new()
        .route("/user", post(create_user))
        .route("/user/:id/permissions", get(get_user_role_permissions))
        .route("/user/:id", delete(delete_user))
        .route("/task", post(create_task))
        .route("/task/:id", put(update_task))
        .route("/task/:id/:user_id", delete(delete_task))
        .layer(Extension(Arc::new(pool)));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
