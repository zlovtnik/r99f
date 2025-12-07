# r99f - Rust WASM + PostgreSQL SPA

A modern single-page application built with Rust compiled to WebAssembly (WASM) for the frontend, Rust/Axum for the backend, and PostgreSQL for data persistence.

## 🦀 Tech Stack

- **Frontend**: Rust + Leptos (compiled to WASM)
- **Backend**: Rust + Axum
- **Database**: PostgreSQL + SQLx
- **Auth**: JWT tokens with bcrypt password hashing

## 📁 Project Structure

```
r99f/
├── frontend/          # Rust WASM frontend (Leptos)
│   ├── src/
│   │   ├── components/
│   │   ├── pages/
│   │   ├── services/
│   │   └── models/
│   └── Trunk.toml
├── backend/           # Rust API server (Axum)
│   ├── src/
│   │   ├── handlers/
│   │   ├── services/
│   │   ├── models/
│   │   ├── db/
│   │   └── middleware/
│   └── migrations/
└── docker-compose.yml
```

## 🚀 Getting Started

### Prerequisites

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

### Development

**1. Start PostgreSQL**
```bash
docker run --name postgres \
  -e POSTGRES_USER=app_user \
  -e POSTGRES_PASSWORD=app_password \
  -e POSTGRES_DB=app_db \
  -p 5432:5432 \
  -d postgres:16
```

**2. Configure Database Connection**

Set the `DATABASE_URL` environment variable so the backend can connect:

```bash
# Option A: Export in terminal
export DATABASE_URL="postgres://app_user:app_password@localhost:5432/app_db"

# Option B: Create a .env file in the backend directory
echo 'DATABASE_URL="postgres://app_user:app_password@localhost:5432/app_db"' > backend/.env
```

**3. Start Backend (Terminal 1)**
```bash
cd backend
sqlx migrate run
cargo watch -x run
# Runs on http://localhost:3000
```

**3. Start Frontend (Terminal 2)**
```bash
cd frontend
trunk serve
# Runs on http://localhost:8080
```

### Production (Docker)

The `docker-compose.yml` defines three services: **postgres** (PostgreSQL 16 database on :5432), **backend** (Rust API on :3000), and **frontend** (nginx serving WASM on :80). A `postgres_data` volume persists database data. The backend waits for postgres to be healthy before starting, and the frontend depends on the backend.

Required environment variables are set in docker-compose.yml with defaults, but for production you should override `JWT_SECRET` and database credentials. Copy `.env.example` to `.env` in the project root (or set vars inline) before running.

```bash
docker-compose up -d
# Frontend: http://localhost
# Backend API: http://localhost:3000
```

## 🔌 API Endpoints

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | `/api/health` | Health check | No |
| POST | `/api/auth/register` | Register user | No |
| POST | `/api/auth/login` | Login | No |
| POST | `/api/auth/refresh` | Refresh token | Yes |
| GET | `/api/users` | List all users | Yes |
| GET | `/api/users/me` | Get current user | Yes |
| GET | `/api/users/:id` | Get user by ID | Yes |
| PUT | `/api/users/:id` | Update user | Yes |
| DELETE | `/api/users/:id` | Delete user | Yes |

## 🔐 Authentication

The API uses JWT tokens for authentication. Include the token in requests:

```
Authorization: Bearer <your-token>
```

## 📜 License

MIT
