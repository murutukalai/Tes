Yes! Here's a guide on implementing API authentication in an Axum web application using PostgreSQL in Rust.

---

## **1. Dependencies**
Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio", "uuid", "chrono"] }
tower-http = { version = "0.5", features = ["cors"] }
jsonwebtoken = "9"   # For JWT authentication
argon2 = "0.5"       # For password hashing
uuid = { version = "1", features = ["v4"] }
dotenvy = "0.15"
hyper = "1"
```

---

## **2. Database Setup**
### **SQL Schema**
Run this in your PostgreSQL database:

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);
```

---

## **3. Configuration & Database Connection**
### **.env file**
Create a `.env` file:

```
DATABASE_URL=postgres://user:password@localhost/db_name
JWT_SECRET=your_secret_key
```

### **Database Setup in Rust**
Create a `db.rs` file:

```rust
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn create_db_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
```

---

## **4. Models**
### **User Model (`models.rs`)**
```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
```

---

## **5. Authentication Helpers**
### **Password Hashing (`auth.rs`)**
```rust
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash, rand_core::OsRng};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;
use uuid::Uuid;

const JWT_SECRET_ENV: &str = "JWT_SECRET";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt).unwrap();
    hash.to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

pub fn generate_jwt(user_id: Uuid) -> String {
    let secret = env::var(JWT_SECRET_ENV).expect("JWT_SECRET must be set");
    let expiration = chrono::Utc::now().timestamp() as usize + 3600;

    let claims = Claims { sub: user_id, exp: expiration };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn verify_jwt(token: &str) -> Option<Uuid> {
    let secret = env::var(JWT_SECRET_ENV).expect("JWT_SECRET must be set");
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).ok()?;
    Some(decoded.claims.sub)
}
```

---

## **6. API Routes**
### **Auth Handlers (`handlers.rs`)**
```rust
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;
use serde_json::json;
use crate::models::{RegisterUser, LoginUser, User};
use crate::auth::{hash_password, verify_password, generate_jwt};

pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterUser>,
) -> (StatusCode, Json<serde_json::Value>) {
    let password_hash = hash_password(&payload.password);
    let result = sqlx::query!(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
        payload.email,
        password_hash
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => (StatusCode::CREATED, Json(json!({"id": user.id}))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "User already exists"}))),
    }
}

pub async fn login_user(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginUser>,
) -> (StatusCode, Json<serde_json::Value>) {
    let user = sqlx::query!("SELECT id, password_hash FROM users WHERE email = $1", payload.email)
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten();

    if let Some(user) = user {
        if verify_password(&payload.password, &user.password_hash) {
            let token = generate_jwt(user.id);
            return (StatusCode::OK, Json(json!({"token": token})));
        }
    }
    (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"})))
}
```

---

## **7. Protecting Routes**
### **JWT Middleware (`middleware.rs`)**
```rust
use axum::{http::Request, middleware::Next, response::Response, http::StatusCode};
use crate::auth::verify_jwt;

pub async fn auth_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok());
    if let Some(token) = auth_header.and_then(|h| h.strip_prefix("Bearer ")) {
        if verify_jwt(token).is_some() {
            return Ok(next.run(req).await);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
```

---

## **8. Setting Up Axum**
### **Main File (`main.rs`)**
```rust
use axum::{Router, routing::{post, get}, middleware, extract::State};
use dotenvy::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::db::create_db_pool;
use crate::handlers::{register_user, login_user};
use crate::middleware::auth_middleware;

mod db;
mod models;
mod handlers;
mod auth;
mod middleware;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = create_db_pool().await;
    let shared_state = Arc::new(pool);

    let app = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(shared_state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
```

---

## **9. Testing**
- **Register:**  
  ```sh
  curl -X POST http://localhost:3000/register -H "Content-Type: application/json" -d '{"email":"test@example.com", "password":"password"}'
  ```
- **Login:**  
  ```sh
  curl -X POST http://localhost:3000/login -H "Content-Type: application/json" -d '{"email":"test@example.com", "password":"password"}'
  ```

This setup implements authentication using Axum, PostgreSQL, Argon2 password hashing, and JWT-based token authentication. Let me know if you need any modifications!
