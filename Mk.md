The approach depends on what you’re trying to achieve. The original example shows how to customize headers for a WebSocket handshake using an asynchronous API (tokio-tungstenite). In Rust, the [ureq](https://crates.io/crates/ureq) crate is a synchronous (blocking) HTTP client and does not support WebSocket upgrade handshakes. That said, you have two paths:

---

## 1. Synchronous WebSocket Connection with Custom Headers

If your goal is to open a WebSocket connection with custom headers but in a blocking (synchronous) manner, you can use the synchronous API of the [tungstenite](https://crates.io/crates/tungstenite) crate. Its API is very similar to the asynchronous one but runs in a blocking style. For example:

```rust
use tungstenite::{connect, client::IntoClientRequest};
use url::Url;
use http::{HeaderValue};

fn main() {
    let ws_url = Url::parse("wss://example.com/socket").unwrap();

    // Create a custom WebSocket request
    let mut request = ws_url.into_client_request().unwrap();

    // Add custom headers
    let headers = request.headers_mut();
    headers.insert("Authorization", HeaderValue::from_static("Bearer YOUR_TOKEN"));
    headers.insert("Custom-Header", HeaderValue::from_static("SomeValue"));

    // Establish the WebSocket connection with the modified request
    let (mut socket, response) = connect(request).expect("Can't connect");
    println!("Connected with response: {:?}", response);

    // Now you can use `socket` to send and receive messages.
}
```

This example uses `tungstenite::connect` (a blocking function) and is equivalent in spirit to the asynchronous example, but without using Tokio. Notice that we still construct the HTTP request with custom headers before initiating the WebSocket handshake.

---

## 2. Making an HTTP Request with Custom Headers Using ureq

If instead you meant that you want to perform a standard HTTP request (not a WebSocket connection) with custom headers using [ureq](https://crates.io/crates/ureq), you can do that as follows:

```rust
fn main() {
    // Make a GET request to an HTTP endpoint with custom headers
    let response = ureq::get("https://example.com/api")
        .set("Authorization", "Bearer YOUR_TOKEN")
        .set("Custom-Header", "SomeValue")
        .call();

    if response.ok() {
        let body = response.into_string().unwrap();
        println!("Response: {}", body);
    } else {
        println!("Request failed: {}", response.status());
    }
}
```

This code uses ureq’s fluent API to set headers and perform a blocking HTTP GET request.

---

## Which One to Use?

- **For WebSocket connections:**  
  Use the synchronous tungstenite API (first example).  
- **For regular HTTP requests:**  
  Use ureq (second example).

Since ureq doesn’t support WebSocket upgrade handshakes, it isn’t a drop‑in replacement for tokio‑tungstenite if your goal is to open a WebSocket connection with custom headers.

If you need further clarification or have a different scenario in mind, let me know!

______________________________________________________________________________________________________________

To add an authentication header to a WebSocket connection using `tokio_tungstenite`, you can pass custom headers in the `connect_async` method. This can be done by creating a `Headers` object and adding it to the request using the `header` field.

Here's how you can modify your code to include an authentication header (for example, a Bearer token):

1. Add the `reqwest` dependency in your `Cargo.toml` to handle headers:
   ```toml
   [dependencies]
   tokio = { version = "1", features = ["full"] }
   tokio-tungstenite = "0.15"
   reqwest = "0.11"  # Add this for headers
   ```

2. Use the `HeaderMap` from `reqwest` to add the authentication header.

Here's an updated version of your code:

```rust
use tokio_tungstenite::{connect_async, tungstenite::Message};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

async fn connect_with_auth(url: &str, token: &str) {
    // Create headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());

    // Pass headers as part of the WebSocket connection
    let request = reqwest::Request::new(
        reqwest::Method::GET,
        url.parse().unwrap(),
    ).headers(headers);

    // Establish the connection with the authentication header
    if let Ok((conn, _)) = connect_async(request).await {
        // WebSocket connection is established here
        println!("WebSocket connection established.");
    } else {
        println!("Failed to connect.");
    }
}
```

### Key Points:
- `reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION}` is used to build the headers, including the authentication token.
- The token is passed as a Bearer token in the header.
- You pass the `HeaderMap` as part of the request when calling `connect_async`.

Make sure that the WebSocket server you are connecting to supports authentication via headers. If you have a specific token or authentication method, adjust the header creation accordingly.

Let me know if you need further clarification!
