Here is a **Rust** example for sending a **raw email** using the **AWS SDK for SESv2**:

---

### **Prerequisites**
1. Install required dependencies:
   ```sh
   cargo add aws-config aws-sdk-sesv2 tokio base64
   ```
2. Configure AWS credentials using `aws configure` or environment variables.

---

### **Rust Example: Sending a Raw Email with SESv2**
```rust
use aws_sdk_sesv2::{Client, Error};
use aws_sdk_sesv2::types::{RawMessage, EmailContent, Destination};
use base64::encode;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load AWS SDK configuration
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let from_email = "your-email@example.com"; // Must be verified in SES
    let to_email = "recipient@example.com";

    // Create raw email in MIME format
    let raw_email = format!(
        "From: {}\r\nTo: {}\r\nSubject: Test Email from AWS SESv2\r\n\
        MIME-Version: 1.0\r\nContent-Type: text/plain; charset=UTF-8\r\n\r\n\
        Hello,\r\n\r\nThis is a test email sent using AWS SESv2 and Rust SDK.\r\n\r\nBest regards,\r\nYour Name",
        from_email, to_email
    );

    // Encode email in Base64 as required by SES
    let raw_message = RawMessage::builder()
        .data(encode(raw_email))
        .build();

    // Create email content with raw message
    let email_content = EmailContent::builder()
        .raw(raw_message)
        .build();

    // Set recipient
    let destination = Destination::builder()
        .to_addresses(to_email)
        .build();

    // Send the email
    let response = client.send_email()
        .from_email_address(from_email)
        .destination(destination)
        .content(email_content)
        .send()
        .await?;

    println!("Email sent successfully: {:?}", response);

    Ok(())
}
```

---

### **Key Features**
- Uses **async/await** with `tokio` for non-blocking execution.
- **MIME-formatted raw email** for custom email content.
- Uses `base64::encode()` because AWS SES requires Base64 encoding.
- Requires **verified sender email** in AWS SES.

Would you like help with adding **attachments** or handling errors more robustly?
