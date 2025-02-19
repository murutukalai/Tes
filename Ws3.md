To keep the WebSocket connection alive and detect disconnections, you can use **ping-pong messages**. Here’s an improved version of the WebSocket client that sends periodic **ping** frames and expects **pong** responses.

---

### **How It Works:**
1. **Sends a ping every 10 seconds** (using `tokio::time::interval`).
2. **Listens for pong responses** to verify the connection is alive.
3. **Reconnects if no pong response is received** within a timeout period.

---

### **WebSocket Client with Ping-Pong & Reconnection**
```rust
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::time::{sleep, timeout, Duration, Instant};
use url::Url;

async fn connect_and_listen(url: &str) {
    let mut retry_delay = Duration::from_secs(1);

    loop {
        println!("Connecting to WebSocket...");

        match connect_async(Url::parse(url).unwrap()).await {
            Ok((mut ws_stream, _)) => {
                println!("Connected successfully!");
                retry_delay = Duration::from_secs(1); // Reset retry delay
                
                let mut ping_interval = tokio::time::interval(Duration::from_secs(10));
                let mut last_pong = Instant::now();

                loop {
                    tokio::select! {
                        Some(message) = ws_stream.next() => {
                            match message {
                                Ok(Message::Pong(_)) => {
                                    println!("Received Pong!");
                                    last_pong = Instant::now(); // Update last pong time
                                }
                                Ok(Message::Text(text)) => {
                                    println!("Received message: {}", text);
                                }
                                Ok(_) => {} // Handle other message types if needed
                                Err(err) => {
                                    println!("WebSocket error: {:?}", err);
                                    break; // Exit loop to trigger reconnection
                                }
                            }
                        }
                        _ = ping_interval.tick() => {
                            if last_pong.elapsed() > Duration::from_secs(20) {
                                println!("Pong timeout, reconnecting...");
                                break; // No pong received in 20s → reconnect
                            }
                            if let Err(e) = ws_stream.send(Message::Ping(Vec::new())).await {
                                println!("Failed to send ping: {:?}", e);
                                break; // Error sending ping → reconnect
                            } else {
                                println!("Sent Ping!");
                            }
                        }
                    }
                }
            }
            Err(err) => {
                println!("Connection failed: {:?}", err);
            }
        }

        println!("Reconnecting in {:?}...", retry_delay);
        sleep(retry_delay).await;
        retry_delay = (retry_delay * 2).min(Duration::from_secs(30)); // Exponential backoff
    }
}

#[tokio::main]
async fn main() {
    let url = "wss://example.com/socket";
    connect_and_listen(url).await;
}
```

---

### **Features of This Implementation**
✅ **Ping-Pong Keepalive:**  
- Sends a `ping` every **10 seconds**.  
- Expects a `pong` response within **20 seconds**.  
- If no `pong` is received, it **disconnects and reconnects**.

✅ **Handles Disconnections:**  
- Reconnects **on errors** (network issues, server down, etc.).  
- Uses **exponential backoff** for retries (max 30 seconds).  

✅ **Efficient with `tokio::select!`:**  
- Listens for **incoming messages** and **ping intervals** in parallel.  
- Avoids blocking while waiting for a pong response.  

This ensures the WebSocket **remains alive**, automatically **detects timeouts**, and **reconnects reliably**. Let me know if you need more features!
-------------

No, this solution does **not** detect a network disconnection immediately because:  
- When the network disconnects, the WebSocket may **not close immediately**.  
- `ws_stream.next()` will **not return an error** until the TCP connection times out.  
- The WebSocket might appear **frozen** instead of failing outright.  

### **How to Detect Network Disconnection Faster?**
1. **Use `ping-pong` with a timeout** (already included in the solution).  
2. **Use a TCP timeout (`timeout(ws_stream.next())`)** to detect a frozen connection.  
3. **Handle unexpected silence** by setting a **hard timeout** for responses.  

---

### **Improved Version: Detects Network Disconnects Faster**
```rust
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::time::{sleep, timeout, Duration, Instant};
use url::Url;

async fn connect_and_listen(url: &str) {
    let mut retry_delay = Duration::from_secs(1);

    loop {
        println!("Connecting to WebSocket...");

        match connect_async(Url::parse(url).unwrap()).await {
            Ok((mut ws_stream, _)) => {
                println!("Connected successfully!");
                retry_delay = Duration::from_secs(1); // Reset retry delay
                
                let mut ping_interval = tokio::time::interval(Duration::from_secs(10));
                let mut last_pong = Instant::now();

                loop {
                    tokio::select! {
                        // Read messages with a timeout to detect connection loss
                        msg = timeout(Duration::from_secs(15), ws_stream.next()) => {
                            match msg {
                                Ok(Some(Ok(Message::Pong(_)))) => {
                                    println!("Received Pong!");
                                    last_pong = Instant::now(); // Update last pong time
                                }
                                Ok(Some(Ok(Message::Text(text)))) => {
                                    println!("Received message: {}", text);
                                }
                                Ok(Some(Ok(_))) => {} // Handle other message types if needed
                                Ok(None) | Err(_) | Ok(Some(Err(_))) => {
                                    println!("Connection lost or timeout, reconnecting...");
                                    break;
                                }
                            }
                        }
                        // Send pings at regular intervals
                        _ = ping_interval.tick() => {
                            if last_pong.elapsed() > Duration::from_secs(20) {
                                println!("No Pong received, assuming disconnection...");
                                break;
                            }
                            if let Err(e) = ws_stream.send(Message::Ping(Vec::new())).await {
                                println!("Failed to send ping: {:?}", e);
                                break;
                            } else {
                                println!("Sent Ping!");
                            }
                        }
                    }
                }
            }
            Err(err) => {
                println!("Connection failed: {:?}", err);
            }
        }

        println!("Reconnecting in {:?}...", retry_delay);
        sleep(retry_delay).await;
        retry_delay = (retry_delay * 2).min(Duration::from_secs(30)); // Exponential backoff
    }
}

#[tokio::main]
async fn main() {
    let url = "wss://example.com/socket";
    connect_and_listen(url).await;
}
```

---

### **Why This Works for Network Disconnections**
✅ **Detects sudden network loss faster:**  
- Uses `timeout(ws_stream.next())`, so it **doesn't wait forever** for data.  
- If `ws_stream.next()` **takes too long**, it **forces a reconnect**.  

✅ **Handles connection freeze (silent disconnects):**  
- If **no Pong is received in 20 seconds**, it **assumes the connection is dead**.  
- This ensures the WebSocket doesn't hang indefinitely.

✅ **Recovers with exponential backoff:**  
- Prevents **reconnecting too quickly** and overwhelming the server.  

This method ensures **fast detection of network failures** and **automatic reconnection** without long delays. Try it and let me know if you need improvements!
