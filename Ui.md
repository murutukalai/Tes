Sure! Below is an **updated version** of the code with a **`todos`** table and CRUD functionality. The API supports:

- **Creating a todo**
- **Fetching all todos**
- **Fetching a specific todo**
- **Updating a todo**
- **Deleting a todo**

---

### **1. Update PostgreSQL Table**
Before running the API, create the `todos` table in your PostgreSQL database:

```sql
CREATE TABLE todos (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    completed BOOLEAN DEFAULT FALSE
);
```

---

### **2. Add Dependencies**
Make sure your `Cargo.toml` includes the required dependencies:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tokio-postgres = { version = "0.7", features = ["with-uuid-1", "with-serde_json-1"] }
serde = { version = "1", features = ["derive"] }
utoipa = { version = "4", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "4", features = ["axum"] }
uuid = { version = "1", features = ["serde", "v4"] }
```

---

### **3. Implement the Todo API**
Here’s the complete Rust API using **Axum** and **PostgreSQL**.

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{NoTls, Client};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

/// Define Todo structure
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Todo {
    id: Uuid,
    title: String,
    description: Option<String>,
    completed: bool,
}

/// Define request payload for creating/updating a Todo
#[derive(Debug, Deserialize, ToSchema)]
struct TodoRequest {
    title: String,
    description: Option<String>,
    completed: Option<bool>,
}

/// PostgreSQL Database State
struct AppState {
    db: Client,
}

/// Handler: Create a new Todo
async fn create_todo(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<TodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    let mut db = state.lock().await;
    let id = Uuid::new_v4();
    let completed = payload.completed.unwrap_or(false);

    let query = "INSERT INTO todos (id, title, description, completed) VALUES ($1, $2, $3, $4)";
    db.db.execute(query, &[&id, &payload.title, &payload.description, &completed])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Todo {
        id,
        title: payload.title,
        description: payload.description,
        completed,
    }))
}

/// Handler: Get all Todos
async fn get_todos(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let mut db = state.lock().await;
    let rows = db.db.query("SELECT id, title, description, completed FROM todos", &[])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todos = rows.iter().map(|row| Todo {
        id: row.get(0),
        title: row.get(1),
        description: row.get(2),
        completed: row.get(3),
    }).collect();

    Ok(Json(todos))
}

/// Handler: Get a single Todo by ID
async fn get_todo(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, StatusCode> {
    let mut db = state.lock().await;

    let row = db.db.query_one("SELECT id, title, description, completed FROM todos WHERE id = $1", &[&id])
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(Todo {
        id: row.get(0),
        title: row.get(1),
        description: row.get(2),
        completed: row.get(3),
    }))
}

/// Handler: Update a Todo
async fn update_todo(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<TodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    let mut db = state.lock().await;
    let completed = payload.completed.unwrap_or(false);

    let query = "UPDATE todos SET title = $1, description = $2, completed = $3 WHERE id = $4";
    let result = db.db.execute(query, &[&payload.title, &payload.description, &completed, &id])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(Todo {
        id,
        title: payload.title,
        description: payload.description,
        completed,
    }))
}

/// Handler: Delete a Todo
async fn delete_todo(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut db = state.lock().await;

    let query = "DELETE FROM todos WHERE id = $1";
    let result = db.db.execute(query, &[&id])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// OpenAPI Documentation
#[derive(OpenApi)]
#[openapi(
    paths(create_todo, get_todos, get_todo, update_todo, delete_todo),
    components(schemas(Todo, TodoRequest))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Connect to PostgreSQL
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=password dbname=test",
        NoTls,
    )
    .await
    .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    let state = Arc::new(Mutex::new(AppState { db: client }));

    let app = Router::new()
        .route("/todos", post(create_todo).get(get_todos))
        .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
        .with_state(state)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Server running on http://{}", addr);
    println!("Swagger UI at http://{}/swagger-ui", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

### **4. Running the Application**
1. Start your PostgreSQL database.
2. Run the application:
   ```sh
   cargo run
   ```
3. Visit the API:
   - API Base URL: [http://localhost:3000/todos](http://localhost:3000/todos)
   - Swagger UI: [http://localhost:3000/swagger-ui](http://localhost:3000/swagger-ui)

---

### **5. Example API Requests**
#### **Create a Todo**
```sh
curl -X POST http://localhost:3000/todos \
     -H "Content-Type: application/json" \
     -d '{"title": "Learn Rust", "description": "Practice Axum with PostgreSQL"}'
```

#### **Get All Todos**
```sh
curl -X GET http://localhost:3000/todos
```

#### **Get a Single Todo**
```sh
curl -X GET http://localhost:3000/todos/<UUID>
```

#### **Update a Todo**
```sh
curl -X PUT http://localhost:3000/todos/<UUID> \
     -H "Content-Type: application/json" \
     -d '{"title": "Master Rust", "description": "Deep dive into Axum", "completed": true}'
```

#### **Delete a Todo**
```sh
curl -X DELETE http://localhost:3000/todos/<UUID>
```

---

This provides a **fully functional TODO API with OpenAPI docs**, using **Axum** and **PostgreSQL**. Let me know if you need any improvements! 🚀
