# Kafkey API

A high-performance, secure Rust-based REST API for Identity and Access Management (IAM). Built with **Axum**, **Diesel ORM**, and **PostgreSQL**.

## 🚀 Key Features

- 🔐 **Authentication**: Secure JWT-based auth with Access and Refresh tokens.
- 👥 **RBAC (Role-Based Access Control)**: Assign roles (Admin, User) to manage high-level access.
- 🛡️ **PBAC (Permission-Based Access Control)**: Fine-grained permissions (e.g., `users.read`) for precise resource control.
- 📧 **User Lifecycle**: Email verification, Password reset, and Profile management.
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

The project follows **Clean Architecture** principles to separate concerns and ensure maintainability:

```text
src/
├── api/              # Interface Adapters
│   ├── axum_http/    # HTTP implementation using Axum
│   │   ├── routers/  # Route definitions & Handlers
│   │   ├── middleware/# Auth, CSRF, Rate-Limit, Logging
│   │   └── extractors/# Custom request handlers (ValidatedJson)
├── application/      # Application Business Rules
│   ├── use_cases/    # Application logic orchestrating entities
│   └── dtos/         # Data Transfer Objects for API contracts
├── domain/           # Enterprise Business Rules
│   ├── entities/     # core domain models (User, Role, Permission)
│   └── repositories/ # Traits defining storage interfaces
├── infrastructure/   # Frameworks & Drivers
│   └── database/     # Concrete implementations (PostgreSQL/Diesel)
└── services/         # Domain-agnostic utilities
    └── jwt_service   # Token generation & validation
```

### 🧱 Layer Responsibilities

1.  **API Layer (`src/api`)**: Handles HTTP requests, maps them to DTOs, and delegates to Use Cases. It knows about the web framework (Axum) but nothing about the database.
2.  **Application Layer (`src/application`)**: Contains business logic (Use Cases). It orchestrates the flow of data between the API layer and the Domain layer.
3.  **Domain Layer (`src/domain`)**: The core of the application. Defines entities and repository interfaces (Traits). It has **zero dependencies** on outer layers.
4.  **Infrastructure Layer (`src/infrastructure`)**: Implements external concerns like Database access. It depends on the Domain layer (interfaces) but the Domain layer does not depend on it.

## 🔐 Security Considerations

- Access tokens are transient (15 min); long-lived sessions require Refresh tokens.
- All administrative routes require specific granular permissions (e.g., `users.read`, `roles.update`).
- The system automatically triggers account lockouts after multiple failed login attempts.

## 📜 License

This project is licensed under the MIT License.
