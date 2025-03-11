Sure! Here's a simple example of CRUD operations using MongoDB in Rust with the `mongodb` crate and `tokio` for async execution.

---

### **1. Add Dependencies**
Add the following to your `Cargo.toml`:

```toml
[dependencies]
mongodb = "2.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

---

### **2. Define the Model**
Create a struct that represents your MongoDB document.

```rust
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    email: String,
}
```

---

### **3. Implement CRUD Operations**
Create a Rust program to perform CRUD operations.

```rust
use mongodb::{bson::doc, error::Result, options::ClientOptions, Client, Collection};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to MongoDB
    let client_uri = "mongodb://localhost:27017"; // Change if needed
    let client = Client::with_uri_str(client_uri).await?;
    
    // Get a reference to the database and collection
    let db = client.database("test_db");
    let collection: Collection<User> = db.collection("users");

    // Create a new user
    let user = User {
        id: None,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    let insert_result = collection.insert_one(user, None).await?;
    println!("Inserted user ID: {:?}", insert_result.inserted_id);

    // Read (Find) all users
    let cursor = collection.find(None, None).await?;
    println!("Users:");
    for result in cursor {
        let user: User = result?;
        println!("{:?}", user);
    }

    // Update a user (using the inserted ID)
    if let Some(user_id) = insert_result.inserted_id.as_object_id() {
        let update_result = collection
            .update_one(
                doc! { "_id": user_id },
                doc! { "$set": { "email": "updated@example.com" } },
                None,
            )
            .await?;
        println!("Updated {} document(s)", update_result.modified_count);
    }

    // Delete a user
    if let Some(user_id) = insert_result.inserted_id.as_object_id() {
        let delete_result = collection
            .delete_one(doc! { "_id": user_id }, None)
            .await?;
        println!("Deleted {} document(s)", delete_result.deleted_count);
    }

    Ok(())
}
```

---

### **4. Running the Code**
Make sure you have MongoDB running locally, then run:

```sh
cargo run
```

---

### **Explanation of CRUD Operations**
1. **Create**: Insert a new document into the `users` collection.
2. **Read**: Retrieve all users and print them.
3. **Update**: Modify the `email` field of a user using `_id`.
4. **Delete**: Remove the user using `_id`.

Would you like me to add error handling or struct-based CRUD functions?
