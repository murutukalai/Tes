Here’s a step-by-step guide on how to integrate **Two-Factor Authentication (2FA)** using an **Authenticator app** (like Google Authenticator or Authy) in an **Axum** web app.

---

## **1. Install Dependencies**
Add the required dependencies in your `Cargo.toml`:

```toml
[dependencies]
axum = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
totp-rs = "5.5"
data-encoding = "2.4"
qrcodegen = "1.8"
rand = "0.8"
tokio = { version = "1", features = ["full"] }
```

---

## **2. Generate a Secret Key for 2FA**
Each user needs a unique secret key for 2FA. This secret should be stored securely in your database.

```rust
use totp_rs::{Algorithm, Secret, TOTP};
use data_encoding::BASE32;
use rand::Rng;

// Generate a new TOTP secret for a user
fn generate_secret() -> String {
    let secret_bytes: [u8; 32] = rand::thread_rng().gen();
    BASE32.encode(&secret_bytes)
}
```

Store this secret in your database along with the user’s ID.

---

## **3. Generate QR Code for Google Authenticator**
The QR code contains a URI that users can scan using an authenticator app.

```rust
use qrcodegen::{QrCode, QrCodeEcc};

// Generate a QR code for the user's TOTP secret
fn generate_qr_code(secret: &str, username: &str, issuer: &str) -> Vec<u8> {
    let uri = format!(
        "otpauth://totp/{issuer}:{username}?secret={secret}&issuer={issuer}&algorithm=SHA1&digits=6&period=30",
        issuer = issuer,
        username = username,
        secret = secret
    );

    let qr = QrCode::encode_text(&uri, QrCodeEcc::Medium).unwrap();
    qr.to_svg_string(4) // Returns SVG string
}
```

---

## **4. Verify OTP Code from the User**
Once the user has set up 2FA, they will enter a 6-digit code from their authenticator app. Validate this code using the stored secret.

```rust
fn verify_totp(secret: &str, user_code: &str) -> bool {
    let totp = TOTP::new(Algorithm::SHA1, 6, 30, Secret::Encoded(secret.to_string()).to_bytes().unwrap()).unwrap();
    totp.check(user_code, 0)
}
```

---

## **5. Axum Routes for 2FA**
### **(a) Enable 2FA – Generate Secret & QR Code**
Users call this API to get a QR code when enabling 2FA.

```rust
use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct QRResponse {
    secret: String,
    qr_code: String,  // SVG format
}

async fn enable_2fa(Json(payload): Json<Enable2FARequest>) -> Json<QRResponse> {
    let secret = generate_secret();
    let qr_code = generate_qr_code(&secret, &payload.username, "MyApp");
    
    // Store secret in DB (pseudo code)
    // db.save_2fa_secret(payload.user_id, secret);

    Json(QRResponse { secret, qr_code })
}

#[derive(Deserialize)]
struct Enable2FARequest {
    user_id: i32,
    username: String,
}
```

---

### **(b) Verify 2FA Code**
This route checks the OTP code entered by the user.

```rust
#[derive(Deserialize)]
struct Verify2FARequest {
    user_id: i32,
    code: String,
}

#[derive(Serialize)]
struct Verify2FAResponse {
    success: bool,
}

async fn verify_2fa(Json(payload): Json<Verify2FARequest>) -> Json<Verify2FAResponse> {
    // Fetch secret from DB (pseudo code)
    let stored_secret = "SECRET_FROM_DB";  // Replace with actual DB call

    let success = verify_totp(stored_secret, &payload.code);
    Json(Verify2FAResponse { success })
}
```

---

## **6. Secure Your API**
- **Store secrets securely** (hashed if necessary).
- **Implement session management** (JWT or cookies) after successful 2FA.
- **Allow 2FA disabling** with identity verification.

---

## **7. Complete Axum App**
Now, combine all the routes in a single Axum app:

```rust
use axum::{Router, routing::post};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/enable_2fa", post(enable_2fa))
        .route("/verify_2fa", post(verify_2fa));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

---

## **8. Frontend Integration**
- Show the **QR code** from the `/enable_2fa` API.
- Let users enter the **OTP code** and verify it via `/verify_2fa`.

Would you like a working example with database integration?
