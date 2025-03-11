Here’s the equivalent MySQL version of your PostgreSQL commands:

```sql
-- Create the database
CREATE DATABASE demo;

-- Create the user and set the password
CREATE USER 'demo'@'localhost' IDENTIFIED BY 'demo2025';

-- Grant all privileges on the database to the user
GRANT ALL PRIVILEGES ON demo.* TO 'demo'@'localhost';

-- Set timezone (MySQL doesn't allow per-database timezone settings, only session-based)
SET GLOBAL time_zone = 'Asia/Kolkata';  -- Note: MySQL uses 'Asia/Kolkata' instead of 'Asia/Calcutta'

-- Grant additional privileges
GRANT CREATE, USAGE ON *.* TO 'demo'@'localhost';

-- Allow the user to create databases and manage roles (MySQL doesn't have direct equivalents for CREATEROLE)
GRANT CREATE, GRANT OPTION ON *.* TO 'demo'@'localhost';

-- Apply the privileges
FLUSH PRIVILEGES;

-- Set session time zone to UTC (for the current session)
SET time_zone = 'UTC';
```

### **Key Differences Between PostgreSQL and MySQL**
1. **`OWNER TO demo`** – MySQL doesn’t support per-database ownership, so we just grant full privileges.
2. **`SET TIMEZONE`** – MySQL supports `SET GLOBAL time_zone = 'Asia/Kolkata';` but doesn't allow per-database timezone settings.
3. **`CREATEDB` and `CREATEROLE`** – MySQL grants similar privileges via `GRANT CREATE, GRANT OPTION`.

Let me know if you need any adjustments!
