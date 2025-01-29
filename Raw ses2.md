Here’s an example of how to send a raw email with multiple attachments using `aws-sdk-sesv2` in Rust. This example constructs a raw email message in MIME format and sends it using the AWS SDK for Rust.

---

### **Steps to Achieve This:**
1. Construct a raw email in **MIME format**.
2. Attach multiple files using **Base64 encoding**.
3. Send the email using **SESv2 `send_email` API**.

---

### **Code Example:**
```rust
use aws_sdk_sesv2::{Client, Error};
use aws_sdk_sesv2::types::{RawMessage};
use base64::encode;
use mime_guess::from_path;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // Email parameters
    let from_email = "sender@example.com";
    let to_email = "recipient@example.com";
    let subject = "Test Email with Multiple Attachments";
    let body_text = "This is a test email with attachments sent via AWS SESv2.";

    // Attachments
    let attachments = vec!["/path/to/file1.pdf", "/path/to/file2.jpg"];
    
    let raw_email = construct_raw_email(from_email, to_email, subject, body_text, &attachments)?;

    let send_email_request = client.send_email()
        .destination(|d| d.to_addresses(to_email))
        .from_email_address(from_email)
        .content(|c| c.raw(|r| r.data(raw_email)))
        .send()
        .await?;

    println!("Email sent successfully: {:?}", send_email_request);
    
    Ok(())
}

fn construct_raw_email(
    from: &str,
    to: &str,
    subject: &str,
    body_text: &str,
    attachments: &[&str]
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let boundary = "BOUNDARY123456789";
    let mut email = format!(
        "From: {}\r\nTo: {}\r\nSubject: {}\r\nMIME-Version: 1.0\r\nContent-Type: multipart/mixed; boundary=\"{}\"\r\n\r\n",
        from, to, subject, boundary
    );

    // Add the email body
    email.push_str(&format!(
        "--{}\r\nContent-Type: text/plain; charset=\"utf-8\"\r\n\r\n{}\r\n",
        boundary, body_text
    ));

    // Add attachments
    for attachment_path in attachments {
        let path = Path::new(attachment_path);
        let mime_type = from_path(path).first_or_octet_stream();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_content = fs::read(path)?;
        let encoded_content = encode(&file_content);

        email.push_str(&format!(
            "--{}\r\nContent-Type: {}\r\nContent-Disposition: attachment; filename=\"{}\"\r\nContent-Transfer-Encoding: base64\r\n\r\n{}\r\n",
            boundary, mime_type, file_name, encoded_content
        ));
    }

    // End the MIME message
    email.push_str(&format!("--{}--\r\n", boundary));

    Ok(email.into_bytes())
}
```

---

### **Explanation:**
- **Constructs a MIME email**: Uses a `multipart/mixed` boundary to separate the text and attachments.
- **Encodes attachments**: Reads files, converts them to Base64, and includes them in the email.
- **Sends the email via AWS SESv2**: Uses `send_email()` with `RawMessage`.

---

### **Dependencies (`Cargo.toml`)**
```toml
[dependencies]
aws-config = "1.2.0"
aws-sdk-sesv2 = "1.2.0"
tokio = { version = "1", features = ["full"] }
base64 = "0.21"
mime_guess = "2.0"
```

---

### **Setup:**
- Set AWS credentials via environment variables or `~/.aws/credentials`.
- Replace `sender@example.com` and `recipient@example.com` with actual verified SES emails.
- Update the file paths for attachments.

This should work for sending an email with multiple attachments using AWS SESv2 in Rust. Let me know if you need modifications!
