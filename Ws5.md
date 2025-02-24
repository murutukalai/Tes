To **create a client ID on the client side and store it on the server** during a WebSocket connection using **Tokio Tungstenite (client)** and **Axum (server)**, you can pass the client ID through the WebSocket connection URL as a query parameter or in the WebSocket headers.

Here's how you can implement it:

---

### ✅ **Plan**

1. **Client Side (Tokio Tungstenite):**  
   - Generate a unique client ID (e.g., UUID).
   - Send the client ID as a query parameter during the WebSocket connection.

2. **Server Side (Axum):**  
   - Extract the client ID from the query parameters when the client connects.
   - Store the client ID with the WebSocket connection in a shared data structure.

---

### 🔥 **Client Code (Tokio Tungstenite with Client ID)**

Add dependencies in `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-tungstenite = "0.20"
uuid = { version = "1", features = ["v4"] }
```

Client Implementation:

```rust
use tokio_tungstenite::connect_async;
use url::Url;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Generate a unique client ID
    let client_id = Uuid::new_v4().to_string();
    let url = format!("ws://127.0.0.1:3000/ws?client_id={}", client_id);

    // Connect to the WebSocket server with the client ID
    let url = Url::parse(&url).unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected with client ID: {}", client_id);

    let (write, read) = ws_stream.split();

    // Here, you can handle sending/receiving messages as needed
}
```

---

### 🔥 **Server Code (Axum to Handle Client ID)**

Add dependencies in `Cargo.toml`:
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tokio-stream = "0.1"
uuid = "1"
```

Server Implementation:

```rust
use axum::{
    extract::{Query, State, ws::{WebSocket, WebSocketUpgrade, Message}},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use serde::Deserialize;
use tokio::sync::mpsc;

type Clients = Arc<Mutex<HashMap<String, mpsc::UnboundedSender<Message>>>>;

#[derive(Debug, Deserialize)]
struct ConnectParams {
    client_id: String,
}

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(clients.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<ConnectParams>,
    State(clients): State<Clients>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, clients, params.client_id))
}

async fn handle_socket(stream: WebSocket, clients: Clients, client_id: String) {
    let (mut sender, mut receiver) = stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Store client with their unique ID
    clients.lock().unwrap().insert(client_id.clone(), tx);
    println!("Client connected: {}", client_id);

    // Sending messages task
    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if sender.send(message).await.is_err() {
                break;
            }
        }
    });

    // Receiving messages from the client
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                println!("Received from {}: {}", client_id, text);
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    // Remove client on disconnect
    clients.lock().unwrap().remove(&client_id);
    println!("Client disconnected: {}", client_id);
}
```

---

### ⚙️ **Explanation**

1. **Client Side:**
   - Generates a UUID for each connection.
   - Passes it as a query parameter in the WebSocket URL.

2. **Server Side:**
   - Extracts `client_id` from the WebSocket query parameters.
   - Stores the client connection using `client_id` in a `HashMap`.

3. **Message Management:**
   - Sends and receives messages from clients independently.
   - Removes clients upon disconnection.

---

### 🚀 **Benefits of This Approach**
- Unique client identification from the client itself.
- Easy scalability to handle 100+ clients.
- Allows targeting specific clients for messages or pings.

Let me know if you need help with custom message formats or handling specific commands from clients!
