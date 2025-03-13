Here is the SQL schema and sample data to support the RBAC system with users, roles, permissions, and tasks.  

```sql
-- Create tables
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    parent_id INT REFERENCES roles(id) ON DELETE SET NULL
);

CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    action VARCHAR(255) NOT NULL,
    role_id INT REFERENCES roles(id) ON DELETE CASCADE
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    role_id INT REFERENCES roles(id) ON DELETE SET NULL
);

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    owner_id INT REFERENCES users(id) ON DELETE CASCADE
);

-- Insert roles
INSERT INTO roles (name, parent_id) VALUES 
('Admin', NULL),
('Manager', 1),
('Employee', 2);

-- Insert permissions
INSERT INTO permissions (action, role_id) VALUES 
('create_task', 2), -- Manager and above
('edit_task', 2),   -- Manager and above
('delete_task', 1); -- Only Admin

-- Insert users
INSERT INTO users (username, role_id) VALUES 
('admin_user', 1),
('manager_user', 2),
('employee_user', 3);

-- Insert tasks
INSERT INTO tasks (title, description, owner_id) VALUES 
('Task 1', 'This is task 1', 2),
('Task 2', 'This is task 2', 3);
```

This setup:
- Defines roles with a hierarchy (Admin > Manager > Employee).
- Assigns permissions based on roles.
- Creates users and assigns them roles.
- Adds some sample tasks.

Let me know if you need any adjustments!
