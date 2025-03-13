Here's an example of using Redis as a cache in an Axum application with `deadpool-redis`. This example includes:  

1. Setting up `deadpool-redis` as a connection pool.
2. Creating middleware to pass Redis connections to handlers.
3. Implementing a simple cache mechanism.

---

### **1. Add Dependencies**
Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
deadpool-redis = { version = "0.14", features = ["rt_tokio_1"] }
redis = { version = "0.25", features = ["tokio-comp"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

---

### **2. Setup Redis Connection Pool**
Create a `redis.rs` module for handling Redis connections:

```rust
use deadpool_redis::{Config, Pool, Runtime};

pub fn create_redis_pool() -> Pool {
    let cfg = Config::from_url("redis://127.0.0.1:6379");
    cfg.create_pool(Some(Runtime::Tokio1)).expect("Failed to create Redis pool")
}
```

---

### **3. Define Cached Route with Axum**
In your `main.rs`, integrate Redis caching:

```rust
use axum::{
    extract::{Path, State},
    response::Json,
    routing::get,
    Router,
};
use deadpool_redis::Pool;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

mod redis;

#[derive(Clone)]
struct AppState {
    redis_pool: Pool,
}

#[derive(Serialize, Deserialize)]
struct ResponseData {
    message: String,
}

async fn get_cached_data(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Json<ResponseData> {
    let mut conn = state.redis_pool.get().await.expect("Failed to get Redis connection");

    // Try getting data from Redis
    if let Ok(Some(cached_value)) = conn.get::<_, Option<String>>(key.clone()).await {
        return Json(ResponseData {
            message: format!("Cached: {}", cached_value),
        });
    }

    // Simulate data fetch and cache it
    let new_value = format!("Hello, {}", key);
    let _: () = conn.set_ex(key.clone(), &new_value, 60).await.expect("Failed to cache data");

    Json(ResponseData { message: new_value })
}

#[tokio::main]
async fn main() {
    let redis_pool = redis::create_redis_pool();
    let state = Arc::new(AppState { redis_pool });

    let app = Router::new()
        .route("/cache/:key", get(get_cached_data))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

### **4. How It Works**
- **First Request:** Calls `/cache/somekey`, returns `"Hello, somekey"` and stores it in Redis.
- **Subsequent Requests:** Fetches `"Hello, somekey"` from Redis instead of recalculating it.
- **TTL (Time-to-Live):** Cached value expires after 60 seconds.

---

### **5. Running the Server**
Make sure Redis is running locally (`redis-server`), then run:

```sh
cargo run
```

Test the cache:
```sh
curl http://127.0.0.1:3000/cache/user123
```

Let me know if you need modifications!
