
## Assumptions
- [ ] Orgs or users are already authenticated via google or anything
- [ ] For now no idempotency key


## Features Implemented

- [x] Create business and api keys
- [x] Create accounts and retrieve balances
- [x] Transfer, credit, and debit operations
- [x] Secure API access with business-scoped keys

## Todo
- [ ] Idempotency Key
- [ ] Almost done with webhook, few errors to resolve.


## Setup

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
You can use either the cloud pg url or local. Tested on supabase postgres
```

### 3. Run with Docker Compose

```bash
# Start the services
docker-compose up --build

```

The API will be available at `http://localhost:8080`



## 🔑 API Documentation

### Authentication
```

All API endpoints (except business creation) require an API key in the header:

```
Authorization: Bearer your-api-key-here
```

### Endpoints

#### Business Management

**Create Business**
```bash
POST /create_business
Content-Type: application/json

{
  "name": "My Business",
}
```

**Create API Key**
```bash
POST /create_api_key
Content-Type: application/json

{
  "id": "business_id"
}
```

#### Account Management

**Create Account**
```bash
POST /create_account?business_id={business_id}
Authorization: Bearer your-api-key
Content-Type: application/json

{
   "name:" "acc_name"
}
```

**Get Account Balance**
```bash
GET /get_balance?business_id={business_id}
Authorization: Bearer your-api-key
{
   "id:" "acc_id"
}
```

#### Transactions

**Transfer Money**
```bash
POST transfer?business_id={business_id}
Authorization: Bearer your-api-key
Content-Type: application/json

{
  "from_account_id": "acc123",
  "to_account_id": "acc456",
  "amount": "50.00",
}
```

**Credit Account**
```bash
POST /credit?business_id={business_id}
Authorization: Bearer your-api-key
Content-Type: application/json

{
  "to_account_id": "acc123",
  "amount": "100.00",
}
```

**Debit Account**
```bash
POST /debit?business_id={business_id}
Authorization: Bearer your-api-key
Content-Type: application/json

{
  "from_account_id": "acc123",
  "amount": "25.00",
}
```


## 📁 Project Structure

```
src/
├── main.rs              # Application entry point
├── db.rs                # Database connection
├── extractor.rs         # Custom extractors (auth) // idempotency to do
├── dispatcher.rs        # Webhook event dispatcher // not completed
├── generate.rs          # ID and key generation utilities
└── routes/
    ├── mod.rs           # Routes module
    ├── account.rs       # Account management endpoints
    ├── business.rs      # Business management endpoints
    ├── transaction.rs   # Transaction endpoints
    └── webhook.rs       # Webhook endpoints
```
