# Kafkey API

A Rust-based REST API with comprehensive Identity and Access Management (IAM) using Axum, Diesel ORM, and PostgreSQL.

## Features

- 🔐 **JWT Authentication** - Secure token-based authentication with access and refresh tokens
- 👥 **Role-Based Access Control (RBAC)** - Fine-grained permission management
- 🔒 **Secure Password Hashing** - Argon2 password hashing algorithm
- 🗄️ **PostgreSQL Database** - Robust relational database with connection pooling
- 🚀 **Modern Rust Stack** - Built with Axum, Diesel, and Tokio
- 📝 **API Versioning** - All endpoints under `/api/v1` prefix

## Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - Fast async web framework
- **ORM**: [Diesel](https://diesel.rs/) - Type-safe SQL query builder
- **Database**: PostgreSQL with R2D2 connection pooling
- **Authentication**: JWT with argon2 password hashing
- **Runtime**: Tokio async runtime

## Getting Started

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- PostgreSQL 14+ ([Install PostgreSQL](https://www.postgresql.org/download/))
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

### Installation

1. **Clone the repository**

```bash
git clone <repository-url>
cd kafkey-api
```

2. **Set up environment variables**

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://username:password@localhost/kafkey_db
JWT_SECRET=your-super-secret-jwt-key-change-this
JWT_REFRESH_SECRET=your-super-secret-refresh-key-change-this
SERVER_PORT=8080
SERVER_TIMEOUT=30
SERVER_BODY_LIMIT=10
```

3. **Create database**

```bash
createdb kafkey_db
```

4. **Run migrations**

```bash
diesel migration run
```

5. **Build and run**

```bash
cargo build --release
cargo run
```

The server will start on `http://localhost:8080` (or your configured port).

## API Documentation

### Authentication Endpoints

#### Register New User

```bash
POST /api/v1/auth/sign-up
Content-Type: application/json

{
  "username": "johndoe",
  "email": "john@example.com",
  "display_name": "John Doe",
  "password": "SecurePass123!",
  "avatar_image_url": "https://example.com/avatar.jpg" // optional
}
```

Response:

```json
{
  "message": "User registered successfully",
  "user_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

#### Login

```bash
POST /api/v1/auth/login
Content-Type: application/json

{
  "email_or_username": "john@example.com",
  "password": "SecurePass123!"
}
```

Response:

```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIs..."
}
```

#### Refresh Access Token

```bash
POST /api/v1/auth/refresh
Content-Type: application/json

{
  "refresh_token": "eyJhbGciOiJIUzI1NiIs..."
}
```

Response:

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs..."
}
```

### Token Information

- **Access Token**: Valid for 15 minutes, contains user ID, roles, and permissions
- **Refresh Token**: Valid for 7 days, used to obtain new access tokens

## Database Schema

### Core Tables

- **users** - User accounts with credentials
- **roles** - Role definitions (admin, user)
- **permissions** - Permission definitions with resource/action model
- **user_roles** - User-to-role assignments
- **role_permissions** - Role-to-permission assignments

### Default Data

After running migrations, the database includes:

**Roles:**

- `admin` - Full system access
- `user` - Basic user access

**Permissions:**

- User management (create, read, update, delete)
- Role management (create, read, update, delete)
- Permission management (create, read, update, delete)

## Project Structure

```
kafkey-api/
├── src/
│   ├── api/              # API layer (routes, DTOs)
│   │   └── axum_http/
│   ├── application/      # Application logic (use cases)
│   │   └── use_cases/
│   ├── domain/           # Domain models (entities, repository traits)
│   │   ├── entities/
│   │   └── repositories/
│   ├── infrastructure/   # External services (database, etc.)
│   │   └── database/
│   ├── services/         # Shared services (JWT, password)
│   └── config/           # Configuration management
├── migrations/           # Diesel database migrations
├── docs/                 # Additional documentation
└── Cargo.toml
```

## Development

### Running Migrations

```bash
# Run all pending migrations
diesel migration run

# Revert last migration
diesel migration revert

# Regenerate schema
diesel migration redo
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run with logging
RUST_LOG=debug cargo run
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

## Environment Variables

| Variable             | Description                   | Default  |
| -------------------- | ----------------------------- | -------- |
| `DATABASE_URL`       | PostgreSQL connection string  | Required |
| `JWT_SECRET`         | Secret key for access tokens  | Required |
| `JWT_REFRESH_SECRET` | Secret key for refresh tokens | Required |
| `SERVER_PORT`        | HTTP server port              | 8080     |
| `SERVER_TIMEOUT`     | Request timeout in seconds    | 30       |
| `SERVER_BODY_LIMIT`  | Max request body size in MB   | 10       |

## Documentation

- [IAM Implementation Plan](docs/IAM_IMPLEMENTATION_PLAN.md) - Detailed implementation guide
- [IAM Task Breakdown](docs/IAM_TASK.md) - Task checklist
- [IAM Walkthrough](docs/IAM_WALKTHROUGH.md) - Complete implementation walkthrough

## Security Considerations

- **Password Hashing**: Uses Argon2, the winner of the Password Hashing Competition
- **JWT Secrets**: Store in environment variables, never commit to repository
- **Token Expiry**: Access tokens expire after 15 minutes to limit exposure
- **Database**: Use strong passwords and secure connection strings

## License

[Add your license here]

## Contributors

[Add contributors here]
