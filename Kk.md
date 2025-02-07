To connect TypeScript with PostgreSQL, you typically use a PostgreSQL client like **pg** or an ORM like **Prisma** or **TypeORM**. Below are three common approaches:

---

## 1. **Using `pg` (Node-Postgres)**
This is a simple and direct way to connect to PostgreSQL.

### **Installation:**
```sh
npm install pg
```

### **Example Code:**
```ts
import { Client } from 'pg';

const client = new Client({
  user: 'your_username',
  host: 'localhost',
  database: 'your_database',
  password: 'your_password',
  port: 5432, // Default PostgreSQL port
});

async function connectDB() {
  try {
    await client.connect();
    console.log('Connected to PostgreSQL');

    const res = await client.query('SELECT NOW()');
    console.log('Current Time:', res.rows[0]);

    await client.end();
  } catch (err) {
    console.error('Error connecting to the database', err);
  }
}

connectDB();
```

---

## 2. **Using Prisma (Recommended for ORM)**
Prisma provides type safety and a better developer experience.

### **Installation:**
```sh
npm install @prisma/client
npm install --save-dev prisma
```

### **Initialize Prisma:**
```sh
npx prisma init
```

### **Edit `prisma/schema.prisma`:**
```prisma
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

model User {
  id    Int     @id @default(autoincrement())
  name  String
  email String  @unique
}
```

### **Set Environment Variables (`.env`):**
```env
DATABASE_URL="postgresql://your_username:your_password@localhost:5432/your_database"
```

### **Generate Prisma Client:**
```sh
npx prisma generate
```

### **Example Code:**
```ts
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

async function main() {
  const users = await prisma.user.findMany();
  console.log(users);
}

main()
  .catch((e) => console.error(e))
  .finally(() => prisma.$disconnect());
```

---

## 3. **Using TypeORM**
TypeORM is another ORM alternative.

### **Installation:**
```sh
npm install typeorm pg reflect-metadata
```

### **Example Code (`data-source.ts`):**
```ts
import "reflect-metadata";
import { DataSource } from "typeorm";

export const AppDataSource = new DataSource({
  type: "postgres",
  host: "localhost",
  port: 5432,
  username: "your_username",
  password: "your_password",
  database: "your_database",
  synchronize: true,
  logging: false,
  entities: ["src/entity/*.ts"],
});
```

### **Example Entity (`User.ts`):**
```ts
import { Entity, PrimaryGeneratedColumn, Column } from "typeorm";

@Entity()
export class User {
  @PrimaryGeneratedColumn()
  id: number;

  @Column()
  name: string;

  @Column({ unique: true })
  email: string;
}
```

### **Connecting to the Database:**
```ts
import { AppDataSource } from "./data-source";

AppDataSource.initialize()
  .then(async () => {
    console.log("Connected to PostgreSQL");
  })
  .catch((error) => console.log(error));
```

---

## **Which One Should You Use?**
- **Use `pg`** if you need a lightweight, low-level PostgreSQL client.
- **Use Prisma** for a modern, developer-friendly ORM with type safety.
- **Use TypeORM** if you prefer a traditional ORM with entity-based modeling.

Let me know if you need help with any specific setup!
_________

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
