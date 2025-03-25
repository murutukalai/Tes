SQLite automatically creates the database file if it doesn't exist when you open a connection using `Connection::open(DB_PATH)`. However, if you want to explicitly check for the existence of the database file before opening it, you can use Rust's `std::path::Path`.

---

### **Updated Code with Explicit DB File Check**
```rust
use arboard::Clipboard;
use rusqlite::{Connection, params};
use std::{thread, time::Duration, path::Path, fs};

const DB_PATH: &str = "clipboard_history.db"; // SQLite database file

fn main() {
    // Check if the database file exists; if not, create it
    if !Path::new(DB_PATH).exists() {
        println!("Database file not found, creating a new one...");
        fs::File::create(DB_PATH).expect("Failed to create database file");
    }

    // Open the database connection
    let conn = Connection::open(DB_PATH).expect("Failed to open database");
    create_table(&conn);

    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");

    let mut last_clipboard_content = String::new();

    loop {
        if let Ok(content) = clipboard.get_text() {
            if content != last_clipboard_content {
                println!("Clipboard changed: {}", content);
                last_clipboard_content = content.clone();
                save_to_db(&conn, &content);
            }
        }

        thread::sleep(Duration::from_secs(1)); // Check clipboard every second
    }
}

/// Creates the clipboard history table if it doesn't exist
fn create_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .expect("Failed to create table");
}

/// Saves clipboard content to the SQLite database
fn save_to_db(conn: &Connection, content: &str) {
    conn.execute(
        "INSERT INTO clipboard_history (content) VALUES (?1)",
        params![content],
    )
    .expect("Failed to insert clipboard data into database");
}
```

---

### **How This Works:**
1. **Checks if `clipboard_history.db` exists using `Path::new(DB_PATH).exists()`**  
   - If not, it creates the file using `fs::File::create(DB_PATH)`.  
   - This ensures the database file is explicitly created before opening a connection.

2. **Opens SQLite connection and ensures the table exists.**  
3. **Monitors clipboard changes and saves new content.**  

---

### **Why This is Useful:**
- Prevents errors in case the database file is missing.
- Ensures a clean database initialization.
- Avoids unnecessary database recreation.

Would you like to add a feature to **retrieve & display stored clipboard data**?
