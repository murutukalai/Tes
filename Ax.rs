use axum::{
    extract::Request,
    http::HeaderMap,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;

async fn get_browser_info(headers: HeaderMap) -> impl IntoResponse {
    if let Some(user_agent) = headers.get("user-agent") {
        if let Ok(ua_str) = user_agent.to_str() {
            return format!("User-Agent: {}", ua_str);
        }
    }
    "User-Agent header not found".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_browser_info));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
