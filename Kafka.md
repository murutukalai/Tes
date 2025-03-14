### **Webhooks with Kafka in Rust**  

To handle webhooks efficiently, we can use **Kafka** for processing events asynchronously. Instead of handling webhook requests immediately, we publish them to a Kafka topic and process them in the background.  

---

### **Overview**
1. **Webhook Receiver** → Accepts webhooks and pushes them to **Kafka**.
2. **Kafka Consumer** → Listens for webhook events and processes them.

---

## **1. Setup Kafka in Docker**  

First, run a local Kafka instance using Docker:

```sh
docker compose up -d
```

Create a `docker-compose.yml` file:

```yaml
version: '3'
services:
  zookeeper:
    image: confluentinc/cp-zookeeper:latest
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181

  kafka:
    image: confluentinc/cp-kafka:latest
    depends_on:
      - zookeeper
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_LISTENERS: PLAINTEXT://:9092
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
    ports:
      - "9092:9092"
```

---

## **2. Add Dependencies**  

Update `Cargo.toml`:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rdkafka = { version = "0.36", features = ["tokio"] }
```

---

## **3. Webhook Receiver (Axum + Kafka Producer)**  

This server will receive webhook events and push them to Kafka.

```rust
use axum::{routing::post, Json, Router};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;

#[derive(Debug, Deserialize, Serialize)]
struct WebhookEvent {
    event_type: String,
    data: serde_json::Value,
}

// Function to initialize Kafka producer
fn create_producer() -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Failed to create Kafka producer")
}

// Webhook handler that sends event to Kafka
async fn webhook_handler(
    Json(payload): Json<WebhookEvent>,
    producer: axum::extract::Extension<FutureProducer>,
) {
    let topic = "webhooks";
    let key = &payload.event_type;
    let value = serde_json::to_string(&payload).unwrap();

    let record = FutureRecord::to(topic)
        .key(key)
        .payload(&value);

    let send_result = producer.send(record, Duration::from_secs(0)).await;
    match send_result {
        Ok(_) => println!("Webhook event sent to Kafka: {:?}", payload),
        Err(e) => eprintln!("Failed to send webhook to Kafka: {:?}", e),
    }
}

#[tokio::main]
async fn main() {
    let producer = create_producer();
    let app = Router::new()
        .route("/webhook", post(webhook_handler))
        .layer(axum::extract::Extension(producer));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

## **4. Kafka Consumer (Webhook Processor)**  

Now, let's consume events from Kafka and process them.

```rust
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::{ClientConfig, Message as KafkaMessage};
use serde_json::Value;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "webhook-consumer")
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Failed to create Kafka consumer");

    consumer.subscribe(&["webhooks"]).expect("Failed to subscribe");

    println!("Waiting for webhook events...");
    let mut message_stream = consumer.stream();
    
    while let Some(message) = message_stream.next().await {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload() {
                    let event: Value = serde_json::from_slice(payload).expect("Failed to parse JSON");
                    println!("Processed webhook: {:?}", event);
                }
            }
            Err(e) => eprintln!("Error receiving message: {:?}", e),
        }
    }
}
```

---

## **5. Test the Webhook System**
### **Send a Webhook Event**
```sh
curl -X POST http://127.0.0.1:3000/webhook \
     -H "Content-Type: application/json" \
     -d '{"event_type": "user_signup", "data": {"username": "mkjs"}}'
```

### **Expected Output**
- The webhook server logs:
  ```
  Webhook event sent to Kafka: WebhookEvent { event_type: "user_signup", data: Object({"username": String("mkjs")}) }
  ```
- The Kafka consumer logs:
  ```
  Processed webhook: {"event_type": "user_signup", "data": {"username": "mkjs"}}
  ```

---

## **Why Use Kafka for Webhooks?**
✅ **Asynchronous Processing** – The webhook receiver doesn't block while handling requests.  
✅ **Scalability** – Multiple consumers can process different types of events.  
✅ **Durability** – Events are stored and retried if a consumer crashes.  

Would you like additional features like retries or authentication?
