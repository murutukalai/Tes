Here's a custom implementation for tracking **RAM usage**, **request URIs**, and **latency** for every request in an Axum server using OpenTelemetry metrics and OpenObserve:

```rust
use axum::{
    body::Body,
    http::{Request, Uri},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use opentelemetry::{
    metrics::{Counter, Histogram, Meter, ObservableGauge},
    KeyValue,
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use sysinfo::{ProcessExt, System, SystemExt};

// Custom metrics container
#[derive(Clone)]
struct AppMetrics {
    meter: Meter,
    ram_usage: ObservableGauge<f64>,
    request_counter: Counter<u64>,
    latency_histogram: Histogram<f64>,
}

impl AppMetrics {
    fn new(meter: Meter) -> Self {
        AppMetrics {
            ram_usage: meter
                .f64_observable_gauge("process.memory.usage")
                .with_description("Current RAM usage in MB")
                .init(),
            request_counter: meter
                .u64_counter("http.requests.total")
                .with_description("Total HTTP requests")
                .init(),
            latency_histogram: meter
                .f64_histogram("http.request.duration.seconds")
                .with_description("Request latency in seconds")
                .init(),
            meter,
        }
    }
}

// Middleware to collect metrics
async fn metrics_middleware(
    metrics: Arc<AppMetrics>,
    request: Request<Body>,
    next: Next<Body>,
) -> impl IntoResponse {
    let start = Instant::now();
    let uri = request.uri().clone();

    // Process the request
    let response = next.run(request).await;

    // Calculate latency
    let latency = start.elapsed().as_secs_f64();

    // Get memory usage
    let mut system = System::new();
    system.refresh_all();
    let memory_usage = system
        .process(sysinfo::get_current_pid().unwrap())
        .map(|p| p.memory() as f64 / 1024.0 / 1024.0) // Convert to MB
        .unwrap_or(0.0);

    // Prepare attributes
    let attributes = vec![
        KeyValue::new("uri", uri.path().to_string()),
        KeyValue::new("method", uri.path().to_string()),
        KeyValue::new("status", response.status().as_u16().to_string()),
    ];

    // Record metrics
    metrics.request_counter.add(1, &attributes);
    metrics.latency_histogram.record(latency, &attributes);
    metrics.meter.register_callback(&[metrics.ram_usage.as_any()], move |observer| {
        observer.observe_f64(&metrics.ram_usage, memory_usage, &attributes)
    });

    response
}

#[tokio::main]
async fn main() {
    // Set up OpenTelemetry with OTLP exporter
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://localhost:4317");

    let meter_provider = opentelemetry_sdk::metrics::MeterProvider::builder()
        .with_reader(
            opentelemetry_otlp::new_pipeline()
                .metrics(opentelemetry_sdk::runtime::Tokio)
                .with_exporter(exporter)
                .with_period(Duration::from_secs(5))
                .build()
                .unwrap(),
        )
        .build();

    opentelemetry::global::set_meter_provider(meter_provider);

    // Initialize custom metrics
    let metrics = Arc::new(AppMetrics::new(
        opentelemetry::global::meter("axum_app"),
    ));

    // Create router with metrics middleware
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/health", get(|| async { "OK" }))
        .layer(middleware::from_fn_with_state(
            metrics.clone(),
            |state, request, next| metrics_middleware(state, request, next),
        ));

    // Start server
    let addr = "127.0.0.1:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### Key Features:
1. **RAM Usage Tracking**:
   - Uses `sysinfo` to get process memory usage
   - Reports in MB via `process.memory.usage` gauge

2. **Request Metrics**:
   - **Latency**: `http.request.duration.seconds` (histogram)
   - **Count**: `http.requests.total` (counter)
   - Attributes: URI path, HTTP method, status code

3. **Custom Middleware**:
   - Wraps every request
   - Calculates duration and memory usage
   - Adds contextual attributes

4. **OpenTelemetry Setup**:
   - OTLP exporter configured for OpenObserve
   - Metrics pushed every 5 seconds

### Dependencies (`Cargo.toml`):
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
opentelemetry = { version = "0.21", features = ["metrics"] }
opentelemetry-otlp = { version = "0.14", features = ["metrics"] }
opentelemetry-sdk = "0.21"
sysinfo = "0.29"
tower = "0.4"
```

### OpenObserve Configuration:
1. Run OpenObserve (Docker):
   ```bash
   docker run -d -p 5080:5080 -p 4317:4317 openobserve/openobserve:latest
   ```

2. View metrics in the OpenObserve UI:
   - Go to `http://localhost:5080`
   - Query metrics like `process.memory.usage` or `http.request.duration.seconds`

### Advanced Usage:
- Add more attributes (user agent, client IP)
- Track error rates separately
- Add request/response size metrics
- Configure buckets for latency histogram:
  ```rust
  .with_explicit_bucket_boundaries(vec![
      0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0
  ])
  ```

This implementation provides granular per-request metrics while being efficient through OpenTelemetry's batching and aggregation capabilities.
