### **Using `sqlx` Instead of `rusqlite`**  
`sqlx` is an async, compile-time verified SQL library for Rust. It works with SQLite, PostgreSQL, and MySQL.

---

### **Install Dependencies**
Add the following to your `Cargo.toml`:
```toml
[dependencies]
arboard = "3.2"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }
```
- `tokio` is required for async operations.
- `sqlx` is configured for SQLite.

---

### **Rust Program Using `sqlx`**
```rust
use arboard::Clipboard;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{path::Path, time::Duration};
use tokio::time::sleep;

const DB_PATH: &str = "clipboard_history.db"; // SQLite database file

#[tokio::main]
async fn main() {
    // Ensure the database file exists
    if !Path::new(DB_PATH).exists() {
        println!("Database file not found, creating a new one...");
        std::fs::File::create(DB_PATH).expect("Failed to create database file");
    }

    // Initialize database connection
    let db_url = format!("sqlite://{}", DB_PATH);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    create_table(&pool).await;

    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");

    let mut last_clipboard_content = String::new();

    loop {
        if let Ok(content) = clipboard.get_text() {
            if content != last_clipboard_content {
                println!("Clipboard changed: {}", content);
                last_clipboard_content = content.clone();
                save_to_db(&pool, &content).await;
            }
        }

        sleep(Duration::from_secs(1)).await; // Check clipboard every second
    }
}

/// Creates the clipboard history table if it doesn't exist
async fn create_table(pool: &Pool<Sqlite>) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS clipboard_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(pool)
    .await
    .expect("Failed to create table");
}

/// Saves clipboard content to the SQLite database
async fn save_to_db(pool: &Pool<Sqlite>, content: &str) {
    sqlx::query("INSERT INTO clipboard_history (content) VALUES (?1)")
        .bind(content)
        .execute(pool)
        .await
        .expect("Failed to insert clipboard data into database");
}
```

---

### **Key Changes in This Version**
✅ **Uses `sqlx` for SQLite interaction** (instead of `rusqlite`).  
✅ **Async operations with `tokio`** for better efficiency.  
✅ **Connection pooling** using `SqlitePoolOptions`.  
✅ **Explicit database file check before opening.**  

---

### **Why `sqlx`?**
- **Asynchronous**: More efficient for larger applications.
- **Compile-time query verification**: Avoids runtime errors.
- **Connection pooling**: Optimized for performance.

Would you like additional features like **searching, exporting data, or clearing old clipboard entries**?
