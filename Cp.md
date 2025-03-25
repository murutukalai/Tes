If you want to capture any content that the user copies to the clipboard anywhere in the system, you need to monitor the clipboard. You can achieve this using Rust with the [`arboard`](https://docs.rs/arboard/latest/arboard/) or [`clipboard`](https://docs.rs/clipboard/latest/clipboard/) crates.

### **Solution Using `arboard`**
The `arboard` crate provides a cross-platform clipboard interface.

#### **Install the dependency:**
Add this to your `Cargo.toml`:
```toml
[dependencies]
arboard = "3.2"
```

#### **Rust Program to Monitor Clipboard Changes:**
```rust
use arboard::Clipboard;
use std::{thread, time::Duration};

fn main() {
    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");

    let mut last_clipboard_content = String::new();

    loop {
        if let Ok(content) = clipboard.get_text() {
            if content != last_clipboard_content {
                println!("Clipboard changed: {}", content);
                last_clipboard_content = content;
            }
        }

        thread::sleep(Duration::from_secs(1)); // Check clipboard every second
    }
}
```

### **Explanation:**
1. The program continuously checks the clipboard every second.
2. If the clipboard content changes, it prints the new copied content.
3. It avoids duplicate prints by comparing the last copied content.

#### **Limitations:**
- Runs as a polling loop (not event-driven).
- Requires running in the background.
- May not work in sandboxed environments like Wayland (Linux).

Would you like an event-driven approach for better efficiency?
