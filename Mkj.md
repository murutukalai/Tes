Got it! For **metrics** with OpenTelemetry in a Rust Axum server targeting OpenObserve, here's a concise example:

---

### 1. **Add Dependencies** (`Cargo.toml`)
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
opentelemetry = { version = "0.21", features = ["metrics", "rt-tokio"] }
opentelemetry-otlp = { version = "0.14", features = ["metrics", "tokio"] }
opentelemetry-semantic-conventions = "0.13"
metrics = "0.21"
metrics-exporter-opentelemetry = "0.11"
tower-http = { version = "0.5", features = ["metrics"] }
```

---

### 2. **Set Up OpenTelemetry Metrics**
```rust
use axum::{Router, routing::get, response::IntoResponse};
use opentelemetry::global;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::metrics::MeterProvider;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Configure OTLP exporter for metrics
    let metrics_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://localhost:4317"); // OpenObserve OTLP endpoint

    // Build MeterProvider
    let meter_provider = MeterProvider::builder()
        .with_reader(
            opentelemetry_otlp::new_pipeline()
                .metrics(opentelemetry_sdk::runtime::Tokio)
                .with_exporter(metrics_exporter)
                .with_period(Duration::from_secs(10)) // Export interval
                .build()
                .unwrap(),
        )
        .build();

    global::set_meter_provider(meter_provider);

    // Create a meter for custom metrics
    let meter = global::meter("axum-server");
    let request_counter = meter
        .u64_counter("http.requests.total")
        .with_description("Total HTTP Requests")
        .init();

    // Build Axum app with metrics middleware
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .layer(
            tower_http::metrics::InFlightLayer::new()
                .with_unit("requests")
                .and(tower_http::metrics::LatencyLayer::new()),
        );

    // Start server
    let addr = "127.0.0.1:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    // Increment custom metric
    global::meter("axum-server")
        .u64_counter("http.requests.total")
        .add(1, &[KeyValue::new("endpoint", "/")]);
    "Hello, metrics!"
}

async fn health_check() -> &'static str {
    "OK"
}
```

---

### 3. **Key Components**
1. **OTLP Exporter**: Sends metrics to OpenObserve via `http://localhost:4317`.
2. **MeterProvider**: Configures the metrics pipeline with a 10-second export interval.
3. **Custom Metrics**: Track HTTP requests with a `u64_counter`.
4. **Middleware**: Uses `tower_http::metrics` for automatic HTTP metrics (latency, in-flight requests).

---

### 4. **OpenObserve Configuration**
1. Run OpenObserve (if not already running):
   ```bash
   docker run -d -p 5080:5080 -p 4317:4317 openobserve/openobserve:latest
   ```
2. View metrics in the OpenObserve UI at `http://localhost:5080`.

---

### 5. **Metrics You’ll See**
- **HTTP Metrics** (from `tower_http` middleware):
  - `http_requests_duration_seconds`
  - `http_requests_in_flight`
- **Custom Metrics**:
  - `http.requests.total` (counter)

---

### Notes
- Adjust labels/attributes (e.g., `endpoint="/"`) to enrich your metrics.
- OpenTelemetry metrics in Rust are still evolving – check the [latest docs](https://opentelemetry.io/docs/instrumentation/rust/) for updates.
- Use OpenObserve’s query UI to visualize and alert on metrics.

Let me know if you need help with custom metric types (gauges, histograms) or advanced use cases! 🚀
