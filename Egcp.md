Here's an enhanced version of your **Clipboard History App** with the following new features:  

✅ **Delete Entries** (Removes specific clipboard history items)  
✅ **Copy Back to Clipboard** (Click an entry to copy it back)  
✅ **Export to CSV** (Saves clipboard history as a `.csv` file)  

---

### **Install Dependencies**  
Make sure your `Cargo.toml` includes these dependencies:
```toml
[dependencies]
arboard = "3.2"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }
eframe = "0.27" # GUI framework
csv = "1.3" # For exporting to CSV
```

---

### **Enhanced Clipboard Manager**
```rust
use arboard::Clipboard;
use csv::Writer;
use eframe::egui::{self, Button, CentralPanel, ScrollArea, TextEdit};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{fs::File, path::Path, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task, time::sleep};

const DB_PATH: &str = "clipboard_history.db"; // SQLite database file
const EXPORT_PATH: &str = "clipboard_export.csv"; // Export file

#[tokio::main]
async fn main() {
    // Ensure database file exists
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

    // Shared state for UI & clipboard monitoring
    let clipboard_history = Arc::new(Mutex::new(Vec::new()));

    // Start clipboard monitoring in the background
    let pool_clone = pool.clone();
    let history_clone = clipboard_history.clone();
    task::spawn(async move { monitor_clipboard(pool_clone, history_clone).await });

    // Run the GUI
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Clipboard History",
        options,
        Box::new(|_cc| Box::new(ClipboardApp { pool, clipboard_history })),
    )
    .expect("Failed to start GUI");
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

/// Monitors clipboard and saves new content to the database
async fn monitor_clipboard(pool: Pool<Sqlite>, clipboard_history: Arc<Mutex<Vec<(i32, String)>>>) {
    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
    let mut last_clipboard_content = String::new();

    loop {
        if let Ok(content) = clipboard.get_text() {
            if content != last_clipboard_content {
                println!("Clipboard changed: {}", content);
                last_clipboard_content = content.clone();
                save_to_db(&pool, &content).await;
                update_ui_history(&pool, &clipboard_history).await;
            }
        }

        sleep(Duration::from_secs(1)).await; // Check clipboard every second
    }
}

/// Saves clipboard content to the SQLite database
async fn save_to_db(pool: &Pool<Sqlite>, content: &str) {
    sqlx::query("INSERT INTO clipboard_history (content) VALUES (?1)")
        .bind(content)
        .execute(pool)
        .await
        .expect("Failed to insert clipboard data into database");
}

/// Fetches clipboard history from the database and updates the UI state
async fn update_ui_history(pool: &Pool<Sqlite>, clipboard_history: &Arc<Mutex<Vec<(i32, String)>>>) {
    let rows = sqlx::query!("SELECT id, content FROM clipboard_history ORDER BY id DESC LIMIT 50")
        .fetch_all(pool)
        .await
        .expect("Failed to fetch clipboard history");

    let mut history = clipboard_history.lock().await;
    *history = rows.into_iter().map(|row| (row.id, row.content)).collect();
}

/// Deletes a clipboard entry from the database
async fn delete_entry(pool: &Pool<Sqlite>, id: i32) {
    sqlx::query("DELETE FROM clipboard_history WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await
        .expect("Failed to delete clipboard entry");
}

/// Exports clipboard history to a CSV file
async fn export_to_csv(pool: &Pool<Sqlite>) {
    let rows = sqlx::query!("SELECT content FROM clipboard_history ORDER BY id DESC")
        .fetch_all(pool)
        .await
        .expect("Failed to fetch clipboard history for export");

    let mut writer = Writer::from_writer(File::create(EXPORT_PATH).expect("Failed to create CSV file"));

    for row in rows {
        writer.write_record(&[row.content]).expect("Failed to write to CSV");
    }

    writer.flush().expect("Failed to flush CSV writer");
    println!("Clipboard history exported to {}", EXPORT_PATH);
}

/// Clipboard History App UI
struct ClipboardApp {
    pool: Pool<Sqlite>,
    clipboard_history: Arc<Mutex<Vec<(i32, String)>>>,
}

impl eframe::App for ClipboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let history = self.clipboard_history.clone();
        let pool_clone = self.pool.clone();

        // Run async task to update UI state
        task::spawn(async move {
            update_ui_history(&pool_clone, &history).await;
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Clipboard History");

            // Scrollable clipboard history list
            ScrollArea::vertical().show(ui, |ui| {
                let mut history = history.blocking_lock(); // Get history data
                for (id, entry) in history.iter() {
                    ui.horizontal(|ui| {
                        if ui.button("📋 Copy").clicked() {
                            let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
                            clipboard.set_text(entry.clone()).expect("Failed to copy to clipboard");
                        }
                        if ui.button("❌ Delete").clicked() {
                            let id_copy = *id;
                            let pool_clone = self.pool.clone();
                            task::spawn(async move {
                                delete_entry(&pool_clone, id_copy).await;
                            });
                        }
                        ui.add(TextEdit::singleline(&mut entry.clone()).desired_width(f32::INFINITY));
                    });
                }
            });

            // Export Button
            if ui.button("📂 Export to CSV").clicked() {
                let pool_clone = self.pool.clone();
                task::spawn(async move {
                    export_to_csv(&pool_clone).await;
                });
            }
        });

        ctx.request_repaint(); // Keep updating UI
    }
}
```

---

### **New Features**
✔ **Delete Entries** (`❌ Delete` button)  
✔ **Copy Back to Clipboard** (`📋 Copy` button)  
✔ **Export to CSV** (`📂 Export to CSV` button)  

---

### **How It Works**
1. **Clipboard Monitoring**
   - Runs in the background and updates the database when new content is copied.

2. **GUI with `egui`**
   - Displays clipboard history in a scrollable list.
   - Each entry has **Copy** and **Delete** buttons.
   - Users can **export** all clipboard history to a CSV file.

3. **Database Handling**
   - **Deletes** an entry from the SQLite database when clicking "Delete".
   - **Exports** history to `clipboard_export.csv` when clicking "Export to CSV".
   - **Copies** an entry back to the clipboard when clicking "Copy".

---

### **Next Steps (Optional Enhancements)**
🔹 **Search feature for clipboard history**  
🔹 **Persistent settings (like auto-clear older entries)**  
🔹 **Minimize to system tray support**  

Would you like any of these enhancements?
