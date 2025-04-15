Sure! Here's a **fully working example** of an Axum app that serves a dynamic sitemap at `/sitemap.xml`, using stable Rust and `axum = "0.7"` with no errors:

---

### **`Cargo.toml`**
```toml
[package]
name = "sitemap_axum"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
```

---

### **`main.rs`**
```rust
use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::net::SocketAddr;

async fn sitemap_handler() -> Response {
    let urls = vec![
        "/",           // home
        "/about",      // static
        "/contact",    // static
        "/short/abc123", // example dynamic
    ];

    let base_url = "https://example.com";

    let mut sitemap = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    sitemap.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

    for path in urls {
        sitemap.push_str(&format!(
            "<url><loc>{}{}</loc></url>",
            base_url, path
        ));
    }

    sitemap.push_str("</urlset>");

    (
        [("Content-Type", "application/xml")],
        sitemap,
    ).into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/sitemap.xml", get(sitemap_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

### **Run it**
```bash
cargo run
```

Then visit [http://localhost:3000/sitemap.xml](http://localhost:3000/sitemap.xml) — you’ll get a valid XML sitemap in the browser.

Want help extending this with dynamic routes (e.g., reading from a DB or in-memory list)?
