To **send pings from the server to a specific WebSocket client in Axum without affecting other clients**, you can handle each connection in its own task and use a `tokio::sync::mpsc` channel for communication.

Here's how to approach it:

### ✅ **Plan**
1. **Manage Client Connections Individually:** Each WebSocket connection runs in a separate `tokio` task.
2. **Track Active Connections:** Use something like `Arc<Mutex<HashMap<ClientId, Sender>>>` to track clients.
3. **Send Ping to a Specific Client:** Use the sender to dispatch a ping frame to the target client without affecting others.

---

### 🔥 **Example Code**

```rust
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    routing::get,
    Router,
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{sync::mpsc, time::{self, Duration}};

type Clients = Arc<Mutex<HashMap<u64, mpsc::UnboundedSender<Message>>>>;

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
    State(clients): State<Clients>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, clients))
}

async fn handle_socket(stream: WebSocket, clients: Clients) {
    let (mut sender, mut receiver) = stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Assign a random client ID (in a real app, use something better)
    let client_id = rand::random::<u64>();

    clients.lock().unwrap().insert(client_id, tx);

    // Task to send messages to the client
    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if sender.send(message).await.is_err() {
                break;
            }
        }
    });

    // Task to receive messages from the client
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                println!("Received from {}: {}", client_id, text);
            }
        }
    });

    // Example: Send ping every 10 seconds to this specific client
    let ping_task = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            if let Err(_) = clients.lock().unwrap().get(&client_id).unwrap().send(Message::Ping(vec![])) {
                println!("Failed to send ping to client {}", client_id);
                break;
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
        _ = ping_task => {},
    }

    // Cleanup on disconnect
    clients.lock().unwrap().remove(&client_id);
}
```

---

### ⚙️ **Explanation**
1. **Client Management:** Each connection gets a unique `client_id` and an associated `mpsc::UnboundedSender<Message>` for sending messages.
2. **Individual Ping:** The `ping_task` sends a ping only to the specific client using their `client_id`.
3. **Concurrency:** Each client runs in its own `tokio` task without blocking others.

---

### ✅ **Why This Works Well for 100+ Clients**
- Each client connection runs independently using lightweight `tokio` tasks.
- Only the targeted client receives the ping.
- Efficient and scalable for many concurrent WebSocket connections.

Let me know if you need help integrating this with your existing Axum setup!
