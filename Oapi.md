Here’s how you can create a simple Rust API using **Axum** with **utoipa** to generate OpenAPI specifications and serve them with Swagger UI.

### **Setup Dependencies**
Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
utoipa = { version = "4", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "4", features = ["axum"] }
serde = { version = "1", features = ["derive"] }
```

### **Create an API with OpenAPI Docs**
```rust
use axum::{routing::get, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Message {
    message: String,
}

/// Handler for the `/hello` route
async fn hello() -> Json<Message> {
    Json(Message {
        message: "Hello, Axum with OpenAPI!".to_string(),
    })
}

/// Define OpenAPI documentation
#[derive(OpenApi)]
#[openapi(paths(hello), components(schemas(Message)))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    println!("Swagger UI at http://{}/swagger-ui", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### **How It Works**
1. **API Definition:**
   - The `/hello` endpoint returns a JSON response with a simple message.
   - The `Message` struct represents the response body.

2. **OpenAPI Integration:**
   - The `#[derive(OpenApi)]` macro from `utoipa` generates OpenAPI documentation.
   - It specifies the API paths and schema components.

3. **Serving OpenAPI Docs:**
   - `SwaggerUi::new("/swagger-ui")` adds a web UI to explore the API.
   - OpenAPI JSON is available at `/api-doc/openapi.json`.

### **Running the API**
Run the server with:
```sh
cargo run
```
Then visit:
- API Endpoint: [http://localhost:3000/hello](http://localhost:3000/hello)
- Swagger UI: [http://localhost:3000/swagger-ui](http://localhost:3000/swagger-ui)

Let me know if you need modifications or more features!
