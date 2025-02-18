use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

// Database model
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
struct Message {
    content: String,
    username: String,
}

// Socket.IO handler with DB integration
async fn on_connect(socket: SocketRef, Data(data): Data<serde_json::Value>, db: Arc<PgPool>) {
    info!("New connection: {:?}", socket.id);
    
    // Handle message events
    socket.on("message", move |socket: SocketRef, Data(msg): Data<Message>| {
        let db = db.clone();
        
        async move {
            // Save to PostgreSQL
            let _ = sqlx::query!(
                "INSERT INTO messages (content, username) VALUES ($1, $2)",
                msg.content,
                msg.username
            )
            .execute(&*db)
            .await;
            
            // Broadcast to all clients
            socket.broadcast().emit("message", msg).ok();
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    // Initialize PostgreSQL pool
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@localhost/chat_db")
        .await?;
    
    sqlx::migrate!().run(&db).await?;

    // Socket.IO setup
    let (layer, io) = SocketIo::builder()
        .with_state(Arc::new(db))
        .build_layer();

    io.ns("/", |socket: SocketRef, Data(auth), db: State<Arc<PgPool>>| {
        on_connect(socket, auth, db)
    });

    // Axum router
    let app = Router::new()
        .route("/", get(|| async { "Chat Server" }))
        .layer(layer);

    info!("Starting server on 0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
