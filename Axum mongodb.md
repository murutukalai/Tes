Here’s how you can integrate MongoDB CRUD operations with an **Axum** server in Rust.

---

### **1. Add Dependencies**
Modify your `Cargo.toml` to include:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
mongodb = "2.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.5", features = ["cors", "trace"] }
```

---

### **2. Define the Model**
Define a `User` struct that represents your MongoDB document.

```rust
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}
```

---

### **3. Create MongoDB Service**
This service will handle all CRUD operations.

```rust
use mongodb::{bson::doc, options::ClientOptions, Client, Collection, error::Result};
use axum::{http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Database {
    pub collection: Collection<User>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let client_uri = "mongodb://localhost:27017"; // Change if needed
        let client = Client::with_uri_str(client_uri).await?;
        let db = client.database("test_db");
        let collection = db.collection("users");
        Ok(Self { collection })
    }

    pub async fn create_user(&self, new_user: CreateUser) -> Result<String> {
        let user = User {
            id: None,
            name: new_user.name,
            email: new_user.email,
        };
        let result = self.collection.insert_one(user, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap().to_hex())
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        let cursor = self.collection.find(None, None).await?;
        let users: Vec<User> = cursor.try_collect().await?;
        Ok(users)
    }

    pub async fn update_user(&self, id: String, email: String) -> Result<bool> {
        let obj_id = ObjectId::parse_str(&id)?;
        let result = self
            .collection
            .update_one(doc! { "_id": obj_id }, doc! { "$set": { "email": email } }, None)
            .await?;
        Ok(result.modified_count > 0)
    }

    pub async fn delete_user(&self, id: String) -> Result<bool> {
        let obj_id = ObjectId::parse_str(&id)?;
        let result = self.collection.delete_one(doc! { "_id": obj_id }, None).await?;
        Ok(result.deleted_count > 0)
    }
}
```

---

### **4. Create Axum Handlers**
Define route handlers for CRUD operations.

```rust
use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::{Database, CreateUser};

pub type AppState = Arc<Mutex<Database>>;

// Create a new user
pub async fn create_user(
    State(db): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let db = db.lock().await;
    match db.create_user(payload).await {
        Ok(id) => (StatusCode::CREATED, Json(id)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to create user".to_string())),
    }
}

// Get all users
pub async fn get_users(State(db): State<AppState>) -> impl IntoResponse {
    let db = db.lock().await;
    match db.get_users().await {
        Ok(users) => (StatusCode::OK, Json(users)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to fetch users".to_string())),
    }
}

// Update user
pub async fn update_user(
    State(db): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(email): Json<String>,
) -> impl IntoResponse {
    let db = db.lock().await;
    match db.update_user(id, email).await {
        Ok(updated) => {
            if updated {
                (StatusCode::OK, Json("User updated successfully".to_string()))
            } else {
                (StatusCode::NOT_FOUND, Json("User not found".to_string()))
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to update user".to_string())),
    }
}

// Delete user
pub async fn delete_user(
    State(db): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let db = db.lock().await;
    match db.delete_user(id).await {
        Ok(deleted) => {
            if deleted {
                (StatusCode::OK, Json("User deleted successfully".to_string()))
            } else {
                (StatusCode::NOT_FOUND, Json("User not found".to_string()))
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to delete user".to_string())),
    }
}
```

---

### **5. Setup Axum Router**
Configure routes and start the server.

```rust
use axum::{routing::*, Router};
use db::Database;
use handlers::*;
use std::sync::Arc;
use tokio::sync::Mutex;

mod db;
mod handlers;

#[tokio::main]
async fn main() {
    let db = Database::new().await.expect("Failed to connect to MongoDB");
    let shared_db = Arc::new(Mutex::new(db));

    let app = Router::new()
        .route("/users", post(create_user).get(get_users))
        .route("/users/:id", put(update_user).delete(delete_user))
        .with_state(shared_db);

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

### **6. Running the Server**
Make sure MongoDB is running, then execute:

```sh
cargo run
```

---

### **7. Testing API Endpoints**
You can test the endpoints using **cURL** or **Postman**.

#### **Create a User**
```sh
curl -X POST http://localhost:3000/users \
     -H "Content-Type: application/json" \
     -d '{"name": "Alice", "email": "alice@example.com"}'
```

#### **Get All Users**
```sh
curl -X GET http://localhost:3000/users
```

#### **Update a User**
```sh
curl -X PUT http://localhost:3000/users/USER_ID \
     -H "Content-Type: application/json" \
     -d '"newemail@example.com"'
```

#### **Delete a User**
```sh
curl -X DELETE http://localhost:3000/users/USER_ID
```

---

### **Conclusion**
This setup provides a **RESTful API** with Axum and MongoDB, allowing you to:
✅ Create users  
✅ Retrieve users  
✅ Update user emails  
✅ Delete users  

Would you like WebSocket support or OpenTelemetry integration?
