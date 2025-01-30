To integrate **metrics** with **Axum**, **OpenTelemetry**, and **OpenObserve**, you need to:  

1. **Set up OpenTelemetry metrics exporter**  
2. **Register and collect metrics**  
3. **Expose an endpoint for Prometheus (optional)**  

---

## **1. Add Dependencies**  
Update `Cargo.toml` with the required dependencies:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
opentelemetry = { version = "0.22", features = ["metrics", "rt-tokio"] }
opentelemetry_sdk = { version = "0.22", features = ["metrics", "rt-tokio"] }
opentelemetry-otlp = { version = "0.22", features = ["http-proto", "reqwest-client"] }
opentelemetry-metrics = "0.22"
opentelemetry-prometheus = "0.22"
prometheus = "0.13"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
```

---

## **2. Initialize OpenTelemetry Metrics**
Create a function to initialize the OpenTelemetry **metrics pipeline**:

```rust
use opentelemetry::metrics::{Meter, Unit};
use opentelemetry::global;
use opentelemetry_sdk::metrics::{controllers, processors, selectors};
use opentelemetry_otlp::WithExportConfig;
use std::sync::Arc;
use opentelemetry_sdk::Resource;

fn init_metrics() -> Result<Meter, Box<dyn std::error::Error>> {
    let exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint("http://localhost:5080/api/default"); // OpenObserve OTLP endpoint

    let controller = controllers::basic(
        processors::factory(
            selectors::simple::histogram([1.0, 10.0, 100.0]), // Histogram buckets
            opentelemetry_sdk::metrics::aggregation::default(),
        )
        .with_memory(true),
    )
    .with_exporter(exporter)
    .build()?;

    let meter_provider = opentelemetry_sdk::metrics::MeterProvider::builder()
        .with_resource(Resource::default())
        .with_reader(controller)
        .build();

    global::set_meter_provider(Arc::new(meter_provider));

    let meter = global::meter("axum_app");
    Ok(meter)
}
```

---

## **3. Register Metrics**
Inside your Axum handler, define and increment metrics:

```rust
use axum::{routing::get, Router};
use opentelemetry::metrics::Counter;
use tracing::info;
use std::sync::Arc;

async fn handler(counter: Arc<Counter<u64>>) -> &'static str {
    counter.add(1, &[]); // Increment request counter
    info!("Handling request");
    "Hello, OpenTelemetry Metrics!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let meter = init_metrics()?;
    let counter = Arc::new(meter.u64_counter("http_requests")
        .with_description("Counts the number of HTTP requests")
        .with_unit(Unit::new("requests"))
        .init());

    let app = Router::new().route("/", get({
        let counter = Arc::clone(&counter);
        move || handler(counter)
    }));

    let addr = "127.0.0.1:3000".parse().unwrap();
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```

---

## **4. Optional: Expose Prometheus Metrics**
If you want to **expose metrics in Prometheus format**, you need an HTTP endpoint for scraping:

### **Update Dependencies**
Add:

```toml
opentelemetry-prometheus = "0.22"
prometheus = "0.13"
```

### **Modify `init_metrics`**
Enable Prometheus exporter:

```rust
use opentelemetry_prometheus::PrometheusExporter;
use prometheus::{Encoder, TextEncoder};

fn init_metrics() -> (Meter, PrometheusExporter) {
    let exporter = opentelemetry_prometheus::exporter().build().unwrap();
    let meter = global::meter("axum_app");
    (meter, exporter)
}
```

### **Create `/metrics` Endpoint**
Expose Prometheus metrics:

```rust
async fn metrics_handler(exporter: PrometheusExporter) -> String {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = exporter.registry().gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (meter, exporter) = init_metrics();
    let counter = Arc::new(meter.u64_counter("http_requests").init());

    let app = Router::new()
        .route("/", get({
            let counter = Arc::clone(&counter);
            move || handler(counter)
        }))
        .route("/metrics", get({
            let exporter = exporter.clone();
            move || metrics_handler(exporter)
        }));

    let addr = "127.0.0.1:3000".parse().unwrap();
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```

---

## **5. Run & Verify**
### **Run the application**
```sh
cargo run
```

### **Send requests**
```sh
curl http://127.0.0.1:3000/
```

### **Check OpenObserve**
- Go to your OpenObserve UI  
- Look under **Metrics** for `http_requests`  

### **Check Prometheus (if enabled)**
```sh
curl http://127.0.0.1:3000/metrics
```

---

## **Next Steps**
- Add **histograms** for request duration  
- Use **attributes** (e.g., method, status code)  
- Export to **Grafana for visualization**  

Would you like help setting up **histograms or dashboards**?
