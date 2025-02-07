If you have a PostgreSQL connection URL, you can create a `pg` (node-postgres) client easily using the **`pg`** package. Here's how you can do it in TypeScript:

---

### **1. Install `pg` if you haven't**
```sh
npm install pg
```

---

### **2. Create PostgreSQL Client from URL**
```ts
import { Client } from 'pg';

// Your PostgreSQL connection URL
const DATABASE_URL = "postgres://user:password@localhost:5432/database_name";

// Create a new PostgreSQL client using the URL
const client = new Client({
  connectionString: DATABASE_URL,
  ssl: process.env.NODE_ENV === "production" ? { rejectUnauthorized: false } : false, // Use SSL in production
});

async function connectDB() {
  try {
    await client.connect();
    console.log("Connected to PostgreSQL");

    const res = await client.query("SELECT NOW()");
    console.log("Current Time:", res.rows[0]);

    await client.end();
  } catch (err) {
    console.error("Database connection error:", err);
  }
}

connectDB();
```

---

### **3. Explanation**
- `connectionString: DATABASE_URL` allows `pg` to parse the URL automatically.
- `ssl` is enabled in production but disabled locally.
- `client.query("SELECT NOW()")` is a test query to check the connection.

Let me know if you need modifications!
