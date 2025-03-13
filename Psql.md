Yes, you can manually terminate all active connections to a PostgreSQL database using the following steps:

### 1. **Connect to PostgreSQL as a Superuser**
Log in to PostgreSQL with a superuser (e.g., `postgres`):

```sh
psql -U postgres
```

### 2. **Find Active Connections**
You can check active connections with:

```sql
SELECT pid, datname, usename, application_name, client_addr FROM pg_stat_activity;
```

### 3. **Terminate All Connections to a Specific Database**
Replace `your_database` with your actual database name:

```sql
SELECT pg_terminate_backend(pid) 
FROM pg_stat_activity 
WHERE datname = 'your_database' AND pid <> pg_backend_pid();
```

This will terminate all connections to `your_database` except for your own session.

### 4. **Force Disconnect All Connections (Including Yours)**
If you need to drop a database but active connections are preventing it, you can forcefully disconnect all users and drop the database:

```sql
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = 'your_database';
DROP DATABASE your_database;
```

Alternatively, you can also use `pg_reload_conf()` after modifying `pg_hba.conf` to temporarily block connections.

Let me know if you need help with a specific case!
