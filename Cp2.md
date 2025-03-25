You can modify the program to write clipboard content to a file, appending new content if the file already exists. Here's an updated version of the script:

### **Updated Rust Program**
```rust
use arboard::Clipboard;
use std::{fs::OpenOptions, io::Write, thread, time::Duration};

const FILE_PATH: &str = "clipboard_log.txt"; // File to store clipboard content

fn main() {
    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");

    let mut last_clipboard_content = String::new();

    loop {
        if let Ok(content) = clipboard.get_text() {
            if content != last_clipboard_content {
                println!("Clipboard changed: {}", content);
                last_clipboard_content = content.clone();
                write_to_file(&content);
            }
        }

        thread::sleep(Duration::from_secs(1)); // Check clipboard every second
    }
}

fn write_to_file(content: &str) {
    let mut file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Append to the file
        .open(FILE_PATH)
        .expect("Failed to open file");

    if let Err(e) = writeln!(file, "{}", content) {
        eprintln!("Failed to write to file: {}", e);
    }
}
```

### **How It Works:**
1. Monitors clipboard changes.
2. If new content is detected, it appends it to `clipboard_log.txt`.
3. If the file doesn't exist, it creates a new one.
4. Runs in an infinite loop, checking the clipboard every second.

This script will log everything the user copies into `clipboard_log.txt`. Let me know if you need improvements!
