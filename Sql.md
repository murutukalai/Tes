To randomly pick featured courses from your course table, you can use SQL with `ORDER BY RANDOM()` (PostgreSQL) or `ORDER BY RAND()` (MySQL). Here’s a query for PostgreSQL:

```sql
SELECT * FROM courses ORDER BY RANDOM() LIMIT 5;
```

For MySQL, use:

```sql
SELECT * FROM courses ORDER BY RAND() LIMIT 5;
```

If you're doing this in Rust with SQLx, you can execute:

```rust
let courses = sqlx::query!("SELECT * FROM courses ORDER BY RANDOM() LIMIT 5")
    .fetch_all(&pool)
    .await?;
```

Let me know if you need it for a specific database or framework!
