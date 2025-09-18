# Service-2: Rust Financial API

A robust Rust-based financial service API built with Actix Web, featuring API key authentication, idempotency support, and webhook integration.

## 🚀 Features

- **Account Management**: Create accounts and retrieve balances
- **Transaction Processing**: Transfer, credit, and debit operations
- **Business Management**: Multi-tenant business support
- **API Key Authentication**: Secure API access with business-scoped keys
- **Idempotency Support**: Prevent duplicate operations
- **Webhook Integration**: Event-driven notifications
- **Docker Support**: Easy deployment with Docker Compose

## 🛠️ Tech Stack

- **Backend**: Rust with Actix Web
- **Database**: PostgreSQL with SQLx
- **Containerization**: Docker & Docker Compose
- **Authentication**: Custom API key system
- **Async Runtime**: Tokio

## 📋 Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## 🏃‍♂️ Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/Muhammad-Owais-Warsi/service.git
cd service-2
```

### 2. Set Up Environment

```bash
# Copy environment template
cp .env.example .env

# Edit .env with your configuration (optional for local development)
nano .env
```

### 3. Run with Docker Compose

```bash
# Start the services (PostgreSQL + API)
docker-compose up --build

# Or run in background
docker-compose up -d --build
```

The API will be available at `http://localhost:8080`

### 4. Verify Setup

```bash
# Check if services are running
docker-compose ps

# View logs
docker-compose logs service-2
```

## 🔧 Development Setup

### Local Development (without Docker)

1. **Start PostgreSQL** (using Docker):
   ```bash
   docker run --name postgres-dev -e POSTGRES_PASSWORD=password -e POSTGRES_DB=service2 -p 5432:5432 -d postgres:15
   ```

2. **Set up database**:
   ```bash
   # Apply schema
   psql -h localhost -U postgres -d service2 -f schema.sql
   ```

3. **Update .env**:
   ```bash
   DATABASE_URL=postgresql://postgres:password@localhost:5432/service2
   ```

4. **Run the application**:
   ```bash
   cargo run
   ```

## 🗃️ Database Schema

The service requires the following database tables:

- `businesses`: Store business entities
- `api_keys`: API authentication keys
- `accounts`: User accounts
- `transactions`: Financial transactions
- `webhook_events`: Event queue for webhooks
- `idempotency_keys`: Prevent duplicate operations

Run `schema.sql` to set up the database structure.

## 🔑 API Documentation

### Authentication

All API endpoints (except business creation) require an API key in the header:

```
Authorization: Bearer your-api-key-here
```

### Endpoints

#### Business Management

**Create Business**
```bash
POST /business
Content-Type: application/json

{
  "name": "My Business",
  "email": "contact@mybusiness.com"
}
```

**Create API Key**
```bash
POST /business/{business_id}/api-key
Authorization: Bearer existing-api-key
Content-Type: application/json

{
  "name": "Production Key"
}
```

#### Account Management

**Create Account**
```bash
POST /account?business_id={business_id}
Authorization: Bearer your-api-key
Idempotency-Key: unique-operation-id
Content-Type: application/json

{
  "user_id": "user123",
  "initial_balance": "100.00"
}
```

**Get Account Balance**
```bash
GET /account/{account_id}/balance?business_id={business_id}
Authorization: Bearer your-api-key
```

#### Transactions

**Transfer Money**
```bash
POST /transaction/transfer?business_id={business_id}
Authorization: Bearer your-api-key
Idempotency-Key: unique-operation-id
Content-Type: application/json

{
  "from_account_id": "acc123",
  "to_account_id": "acc456",
  "amount": "50.00",
  "description": "Payment for services"
}
```

**Credit Account**
```bash
POST /transaction/credit?business_id={business_id}
Authorization: Bearer your-api-key
Idempotency-Key: unique-operation-id
Content-Type: application/json

{
  "account_id": "acc123",
  "amount": "100.00",
  "description": "Deposit"
}
```

**Debit Account**
```bash
POST /transaction/debit?business_id={business_id}
Authorization: Bearer your-api-key
Idempotency-Key: unique-operation-id
Content-Type: application/json

{
  "account_id": "acc123",
  "amount": "25.00",
  "description": "Withdrawal"
}
```

## 🔄 Idempotency

The API supports idempotency for safe retries. Include an `Idempotency-Key` header with a unique identifier for each operation:

```bash
Idempotency-Key: txn_20230918_001
```

If the same key is used again, the API will return the cached response instead of processing the operation again.

## 🌐 Production Deployment

### Using Supabase PostgreSQL

1. **Update .env for production**:
   ```bash
   DATABASE_URL=postgresql://postgres:[PASSWORD]@db.[PROJECT-REF].supabase.co:5432/postgres
   ```

2. **Use production compose file**:
   ```bash
   docker-compose -f docker-compose.prod.yml up -d --build
   ```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `RUST_LOG` | Log level (error, warn, info, debug, trace) | `info` |
| `POSTGRES_DB` | Database name (for local dev) | `service2` |
| `POSTGRES_USER` | Database user (for local dev) | `postgres` |
| `POSTGRES_PASSWORD` | Database password (for local dev) | `password` |

## 🧪 Testing

```bash
# Run tests
cargo test

# Run with coverage
cargo test -- --test-threads=1

# Integration tests with Docker
docker-compose up -d postgres
cargo test
docker-compose down
```

## 📁 Project Structure

```
src/
├── main.rs              # Application entry point
├── db.rs                # Database connection
├── extractor.rs         # Custom extractors (auth, idempotency)
├── dispatcher.rs        # Webhook event dispatcher
├── generate.rs          # ID and key generation utilities
└── routes/
    ├── mod.rs           # Routes module
    ├── account.rs       # Account management endpoints
    ├── business.rs      # Business management endpoints
    ├── transaction.rs   # Transaction endpoints
    └── webhook.rs       # Webhook endpoints
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Quick Contribution Guide

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Add tests if applicable
5. Ensure tests pass: `cargo test`
6. Commit your changes: `git commit -m 'Add amazing feature'`
7. Push to the branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🐛 Troubleshooting

### Common Issues

**Database Connection Error**
```
Error: connection failed
```
- Ensure PostgreSQL is running
- Check DATABASE_URL in .env file
- Verify database credentials

**Port Already in Use**
```
Error: Address already in use (os error 98)
```
- Stop other services on port 8080: `docker-compose down`
- Or change port in docker-compose.yml

**Build Errors**
```
Error: could not compile
```
- Ensure Rust 1.70+ is installed: `rustc --version`
- Clean build cache: `cargo clean && cargo build`

### Docker Issues

**Container Won't Start**
```bash
# Check logs
docker-compose logs service-2

# Rebuild containers
docker-compose build --no-cache
```

**Database Init Fails**
```bash
# Check postgres logs
docker-compose logs postgres

# Recreate volumes
docker-compose down -v
docker-compose up --build
```

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/Muhammad-Owais-Warsi/service/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Muhammad-Owais-Warsi/service/discussions)

## 🗺️ Roadmap

- [ ] JWT authentication support
- [ ] Rate limiting
- [ ] API versioning
- [ ] Comprehensive audit logging
- [ ] GraphQL API
- [ ] Real-time notifications
- [ ] Multi-currency support

---

Made with ❤️ by [Muhammad Owais Warsi](https://github.com/Muhammad-Owais-Warsi)