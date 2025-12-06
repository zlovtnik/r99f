# Rust WASM + PostgreSQL Single Page Application - Development Specification

## 1. Project Overview

A single-page application built with Rust compiled to WebAssembly (WASM) for the frontend, with a Rust backend API that connects to PostgreSQL for data persistence.

## 2. Architecture

### 2.1 System Components

**Frontend (Rust → WASM)**
- Compiled Rust code running in the browser via WebAssembly
- Handles UI rendering, user interactions, and client-side logic
- Communicates with backend via REST API or WebSocket

**Backend (Rust API Server)**
- Axum or Actix-web HTTP server
- Business logic and data validation
- Database connection pooling and queries
- Authentication and authorization

**Database (PostgreSQL)**
- Relational data storage
- ACID compliance for data integrity
- Connection managed through connection pool

### 2.2 Technology Stack
```
┌─────────────────────────────────────┐
│         Browser (Client)            │
│  ┌───────────────────────────────┐  │
│  │   Rust WASM (Yew/Leptos)      │  │
│  │   - UI Components             │  │
│  │   - State Management          │  │
│  │   - API Client                │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
              │ HTTP/WS
              ↓
┌─────────────────────────────────────┐
│      Backend Server (Rust)          │
│  ┌───────────────────────────────┐  │
│  │   Axum/Actix-web              │  │
│  │   - Routing                   │  │
│  │   - Middleware                │  │
│  │   - Business Logic            │  │
│  └───────────────────────────────┘  │
│  ┌───────────────────────────────┐  │
│  │   SQLx/Diesel                 │  │
│  │   - Query Builder             │  │
│  │   - Connection Pool           │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
              │ SQL
              ↓
┌─────────────────────────────────────┐
│         PostgreSQL                  │
│  - Tables, Indexes, Constraints     │
│  - Stored Procedures (optional)     │
└─────────────────────────────────────┘
```

## 3. Frontend Specification (Rust WASM)

### 3.1 Framework Options

**Option A: Yew** (React-like)
- Component-based architecture
- Virtual DOM
- Mature ecosystem

**Option B: Leptos** (Modern, fine-grained reactivity)
- Better performance
- Smaller bundle sizes
- Server-side rendering support

### 3.2 Key Dependencies
```toml
[dependencies]
# Framework (choose one)
yew = "0.21"
# OR
leptos = "0.6"

# HTTP client
reqwest = { version = "0.11", features = ["json"] }
gloo-net = "0.5"  # Alternative for Yew

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Utilities
wasm-bindgen = "0.2"
web-sys = "0.3"
js-sys = "0.3"
```

### 3.3 Project Structure
```
frontend/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Root component
│   ├── components/          # UI components
│   │   ├── mod.rs
│   │   ├── header.rs
│   │   ├── footer.rs
│   │   └── ...
│   ├── pages/               # Page components
│   │   ├── mod.rs
│   │   ├── home.rs
│   │   ├── dashboard.rs
│   │   └── ...
│   ├── services/            # API clients
│   │   ├── mod.rs
│   │   └── api.rs
│   ├── models/              # Data models
│   │   ├── mod.rs
│   │   └── user.rs
│   └── utils/               # Utilities
│       └── mod.rs
├── static/                  # Static assets
│   ├── index.html
│   └── styles.css
├── Cargo.toml
└── Trunk.toml               # Trunk build config
```

### 3.4 Build Configuration

Use **Trunk** for building and serving:
```toml
# Trunk.toml
[build]
target = "index.html"
release = false

[watch]
ignore = ["target"]

[serve]
address = "127.0.0.1"
port = 8080
```

## 4. Backend Specification (Rust API)

### 4.1 Framework: Axum (recommended)

Modern, ergonomic, built on Tokio

### 4.2 Key Dependencies
```toml
[dependencies]
# Web framework
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "macros", "migrate", "chrono", "uuid"] }
# OR
diesel = { version = "2.1", features = ["postgres", "r2d2", "chrono"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Environment & Config
dotenvy = "0.15"
config = "0.14"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Security
bcrypt = "0.15"
jsonwebtoken = "9.2"
uuid = { version = "1.6", features = ["v4", "serde"] }

# Date/Time
chrono = { version = "0.4", features = ["serde"] }
```

### 4.3 Project Structure
```
backend/
├── src/
│   ├── main.rs              # Entry point
│   ├── config.rs            # Configuration
│   ├── routes/              # API routes
│   │   ├── mod.rs
│   │   ├── users.rs
│   │   └── ...
│   ├── handlers/            # Request handlers
│   │   ├── mod.rs
│   │   └── user_handler.rs
│   ├── models/              # Domain models
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── db/                  # Database layer
│   │   ├── mod.rs
│   │   ├── pool.rs
│   │   └── repositories/
│   │       └── user_repo.rs
│   ├── middleware/          # Custom middleware
│   │   ├── mod.rs
│   │   └── auth.rs
│   ├── services/            # Business logic
│   │   ├── mod.rs
│   │   └── user_service.rs
│   ├── error.rs             # Error types
│   └── utils/
│       └── mod.rs
├── migrations/              # SQL migrations
│   └── 001_create_users.sql
├── .env                     # Environment variables
├── Cargo.toml
└── sqlx-data.json          # SQLx compile-time checks
```

### 4.4 Database Connection

**Using SQLx (recommended for compile-time safety)**
```rust
// db/pool.rs
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn create_pool(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
```

**Environment Configuration**
```env
# .env
DATABASE_URL=postgresql://user:password@localhost:5432/dbname
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
JWT_SECRET=your-secret-key
RUST_LOG=debug
```

## 5. Database Specification (PostgreSQL)

### 5.1 Connection Configuration
```
Host: localhost (or container name)
Port: 5432
Database: app_db
User: app_user
Password: secure_password
SSL Mode: require (production)
Max Connections: 100
```

### 5.2 Migration Management

**Using SQLx CLI**
```bash
# Install
cargo install sqlx-cli --no-default-features --features postgres

# Create migration
sqlx migrate add create_users_table

# Run migrations
sqlx migrate run

# Revert
sqlx migrate revert
```

### 5.3 Example Schema
```sql
-- migrations/001_create_users.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);

-- Trigger for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

## 6. API Design

### 6.1 RESTful Endpoints
```
GET    /api/health              # Health check
POST   /api/auth/register       # User registration
POST   /api/auth/login          # User login
POST   /api/auth/refresh        # Refresh token
GET    /api/users               # List users (protected)
GET    /api/users/:id           # Get user (protected)
PUT    /api/users/:id           # Update user (protected)
DELETE /api/users/:id           # Delete user (protected)
```

### 6.2 Request/Response Models
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}
```

## 7. Development Workflow

### 7.1 Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk (frontend build tool)
cargo install --locked trunk

# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Install cargo-watch for auto-reload
cargo install cargo-watch
```

### 7.2 Running the Application

**Terminal 1 - PostgreSQL**
```bash
docker run --name postgres \
  -e POSTGRES_USER=app_user \
  -e POSTGRES_PASSWORD=app_password \
  -e POSTGRES_DB=app_db \
  -p 5432:5432 \
  -d postgres:16
```

**Terminal 2 - Backend**
```bash
cd backend
sqlx migrate run
cargo watch -x run
# Runs on http://localhost:3000
```

**Terminal 3 - Frontend**
```bash
cd frontend
trunk serve
# Runs on http://localhost:8080
```

### 7.3 Production Build
```bash
# Frontend
cd frontend
trunk build --release

# Backend
cd backend
cargo build --release

# Docker deployment
docker-compose up -d
```

## 8. Security Considerations

### 8.1 Authentication
- JWT tokens for stateless auth
- Secure password hashing (bcrypt, argon2)
- Token refresh mechanism
- HTTPS only in production

### 8.2 Database Security
- Prepared statements (SQLx/Diesel handles this)
- Least privilege database user
- Connection encryption (SSL/TLS)
- Environment variable secrets

### 8.3 CORS Configuration
```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(["http://localhost:8080".parse().unwrap()])
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(Any);
```

## 9. Testing Strategy

### 9.1 Frontend Tests
```bash
wasm-pack test --headless --firefox
```

### 9.2 Backend Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_create_user() {
        // Integration tests with test database
    }
}
```

### 9.3 Database Tests
- Use a test database
- Run migrations before tests
- Clean up after tests

## 10. Performance Optimization

### 10.1 Frontend
- Code splitting where possible
- Lazy loading of routes
- Minimize WASM binary size with `wasm-opt`
- Use `web-sys` features selectively

### 10.2 Backend
- Connection pooling (configured)
- Database query optimization
- Caching layer (Redis) if needed
- Async/await throughout

### 10.3 Database
- Proper indexing
- Query optimization
- Connection pooling
- Regular VACUUM and ANALYZE

## 11. Monitoring & Logging
```rust
// Backend logging
use tracing::{info, error};
use tracing_subscriber;

tracing_subscriber::fmt()
    .with_env_filter("debug")
    .init();

info!("Server starting on {}", addr);
```

## 12. Deployment

### 12.1 Docker Compose Example
```yaml
version: '3.8'

services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_USER: app_user
      POSTGRES_PASSWORD: app_password
      POSTGRES_DB: app_db
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  backend:
    build: ./backend
    environment:
      DATABASE_URL: postgresql://app_user:app_password@postgres:5432/app_db
    ports:
      - "3000:3000"
    depends_on:
      - postgres

  frontend:
    build: ./frontend
    ports:
      - "80:80"

volumes:
  postgres_data:
```

## 13. Next Steps

1. Initialize both frontend and backend Cargo projects
2. Set up PostgreSQL database
3. Implement basic CRUD operations
4. Add authentication layer
5. Build out UI components
6. Implement business logic
7. Add comprehensive tests
8. Set up CI/CD pipeline
9. Deploy to production environment

---

This specification provides a solid foundation for building a production-ready Rust WASM + PostgreSQL application.