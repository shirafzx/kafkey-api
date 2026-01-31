# Kafkey API

A high-performance, secure Rust-based REST API for Identity and Access Management (IAM). Built with **Axum**, **Diesel ORM**, and **PostgreSQL**.

## 🚀 Key Features

- 🔐 **Authentication**: Secure JWT-based auth with Access and Refresh tokens.
- 🛡️ **PBAC (Permission-Based Access Control)**: Granular permissions system for secure resource management.
- 🛡️ **Advanced Security**
  - **CSRF Protection**: Double Submit Cookie pattern.
  - **Request Validation**: Automated payload validation (e.g., email format, password strength).
  - **Account Lockout**: Brute-force protection.
  - **Token Blacklisting**: Revocation for secure logout.
  - **Global Rate Limiting**: DDoS protection.
- 📋 **API Excellence**:
  - **Standardized Responses**: Consistent envelope structure for success and errors.
  - **camelCase Support**: Seamless integration with modern frontend frameworks.
  - **Request Tracking**: Automated `x-request-id` headers for distributed tracing.
  - **Pagination**: Efficient list processing with `hasNext`/`hasPrev` indicators.

## 🛠️ Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Database**: PostgreSQL with Connection Pooling (R2D2)
- **ORM**: [Diesel](https://diesel.rs/)
- **Auth**: JWT (jsonwebtoken) & Argon2 (argon2)
- **Middleware**: Tower Service layers for tracing, timeout, and limits

## 🏁 Getting Started

### Prerequisites

- Rust 1.70+
- PostgreSQL 14+
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

### Installation

1. **Clone & Setup Environment**

   ```bash
   git clone https://github.com/shirafzx/kafkey-api.git
   cd kafkey-api
   cp .env.example .env # Update with your database credentials
   ```

2. **Initialize Database**

   ```bash
   createdb kafkey_db
   diesel migration run
   ```

3. **Run Application**
   ```bash
   cargo run # Server starts on http://localhost:8080
   ```

## 📖 API Standards

### Standard Response Structure

All API responses follow this consistent format:

```json
{
  "success": true,
  "code": "USER_LOGIN_SUCCESS",
  "message": "Login successful",
  "data": { ... },
  "meta": {
    "requestId": "uuid-v4",
    "timestamp": "2026-01-31T01:23:45Z",
    "version": "1.0"
  }
}
```

### Authentication Example

**POST /api/v1/auth/login**

```json
{
  "emailOrUsername": "admin@example.com",
  "password": "SecurePassword123!"
}
```

## 📂 Documentation

- 📄 [Detailed API Specification](docs/API.md) - Full endpoint lists and examples.
- 📊 [System Diagrams](docs/DIAGRAMS.md) - Visual sequence diagrams for Auth and AuthZ flows.
- 📝 [Development Tasks](TASKS.md) - Roadmap and completed features.

## 🏗️ Project Architecture

```text
src/
├── api/              # API Layer (Routers, Middleware)
├── application/      # Service Layer (Use Cases, DTOs)
├── domain/           # Business Core (Entities, Repositories)
├── infrastructure/   # Technical Impl (PostgreSQL, Schema)
└── services/         # Utilities (JWT, Hashing)
```

## 🔐 Security Considerations

- Access tokens are transient (15 min); long-lived sessions require Refresh tokens.
- All administrative routes require specific granular permissions (e.g., `users.read`, `roles.update`).
- The system automatically triggers account lockouts after multiple failed login attempts.

## 📜 License

This project is licensed under the MIT License.
