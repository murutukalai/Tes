Here's a simple Axum application that implements basic authentication using PostgreSQL (`psql`). The demo will:  

- Use `tokio-postgres` for database interactions.  
- Implement basic authentication using the `headers` feature of `axum`.  
- Verify user credentials against the database.  

---

### **1. Setup Your PostgreSQL Database**
```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

-- Insert a test user (password: "password123")
INSERT INTO users (username, password) VALUES ('testuser', crypt('password123', gen_salt('bf')));
```
---

### **2. Add Dependencies**
Add these dependencies to your `Cargo.toml`:
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tokio-postgres = { version = "0.7", features = ["with-chrono-0_4"] }
hyper = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["auth"] }
bcrypt = "0.15"
serde = { version = "1", features = ["derive"] }
base64 = "0.21"
```

---

### **3. Implement the Axum Server**
```rust
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use bcrypt::verify;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

#[tokio::main]
async fn main() {
    // Connect to the PostgreSQL database
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=yourpassword dbname=authdb", NoTls)
            .await
            .expect("Failed to connect to database");

    // Spawn the database connection handler in a separate task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    // Shared database client
    let shared_client = Arc::new(Mutex::new(client));

    // Define routes with authentication middleware
    let app = Router::new()
        .route("/protected", get(protected_handler))
        .layer(middleware::from_fn_with_state(
            shared_client.clone(),
            auth_middleware,
        ))
        .with_state(shared_client);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

// Middleware to check basic authentication
async fn auth_middleware<B>(
    State(client): State<Arc<Mutex<Client>>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Basic ") {
                let encoded = auth_str.trim_start_matches("Basic ");
                if let Ok(decoded) = general_purpose::STANDARD.decode(encoded) {
                    if let Ok(credentials) = String::from_utf8(decoded) {
                        let parts: Vec<&str> = credentials.splitn(2, ':').collect();
                        if parts.len() == 2 {
                            let username = parts[0];
                            let password = parts[1];

                            if authenticate(username, password, client.clone()).await {
                                return Ok(next.run(req).await);
                            }
                        }
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

// Authenticate user against the database
async fn authenticate(username: &str, password: &str, client: Arc<Mutex<Client>>) -> bool {
    let query = "SELECT password FROM users WHERE username = $1";
    let client = client.lock().await;

    if let Ok(rows) = client.query(query, &[&username]).await {
        if let Some(row) = rows.first() {
            let stored_hash: String = row.get(0);
            return verify(password, &stored_hash).unwrap_or(false);
        }
    }

    false
}

// Handler for the protected route
async fn protected_handler() -> &'static str {
    "Welcome to the protected area!"
}
```

---

### **4. How to Test**
1. **Start the server:**
   ```sh
   cargo run
   ```

2. **Send an authenticated request using `curl`:**
   ```sh
   curl -u testuser:password123 http://127.0.0.1:3000/protected
   ```

   **Expected Response:**
   ```
   Welcome to the protected area!
   ```

3. **Try without credentials:**
   ```sh
   curl http://127.0.0.1:3000/protected
   ```

   **Expected Response:**
   ```
   401 Unauthorized
   ```

---

### **5. Summary**
- We set up a PostgreSQL database with user authentication.
- The Rust server uses Axum with middleware to validate Basic Auth headers.
- Credentials are checked against hashed passwords in the database.
- Unauthorized users receive a `401` status.

This provides a solid foundation for user authentication in Axum using PostgreSQL. Let me know if you need enhancements like JWT or session-based authentication!
