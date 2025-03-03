# Solana Todo Backend

A REST API that interacts with the Solana Todo smart contract, built with Rust and Actix-web.

## Features

- **Authentication**: Secure authentication using Solana wallet signatures and JWT tokens
- **RESTful API**: Full-featured API for creating, reading, updating, and deleting todo items
- **Documentation**: API documentation using OpenAPI/Swagger
- **Rate Limiting**: Basic rate limiting to prevent abuse
- **Docker Support**: Ready for containerization and easy deployment

## API Endpoints

### Authentication

- `POST /api/auth` - Authenticate with Solana wallet

### Todo Operations

- `GET /api/todos` - List all todos for the authenticated wallet
- `POST /api/todos` - Create a new todo
- `PUT /api/todos/:id` - Update a todo
- `DELETE /api/todos/:id` - Delete a todo

## Technologies Used

- **Rust** - Programming language
- **Actix-web** - Web framework
- **Anchor** - Solana framework integration
- **JWT** - Authentication tokens
- **OpenAPI/Swagger** - API documentation
- **Docker** - Containerization

## Prerequisites

- Rust 1.56.0 or later
- Docker and Docker Compose (optional)

## Getting Started

### Local Development

1. Clone the repository
2. Update the `.env` file with your configuration
3. Build and run the project:

```bash
cargo build
cargo run
```

4. Visit the API documentation at: http://localhost:8080/api/docs/

### Using Docker

1. Clone the repository
2. Build and run with Docker Compose:

```bash
docker-compose up -d
```

3. Visit the API documentation at: http://localhost:8080/api/docs/

## Configuration

Configuration is managed through environment variables or the `.env` file:

| Variable | Description | Default |
|----------|-------------|---------|
| SERVER_HOST | Host to bind the server | 127.0.0.1 |
| SERVER_PORT | Port to bind the server | 8080 |
| RUST_LOG | Logging level | info |
| SOLANA_RPC_URL | Solana RPC URL | http://localhost:8899 |
| SOLANA_PROGRAM_ID | Solana program ID | Ct2N3zw5LFiNj5mJ7hN2c4umze2pAWNjfYqazZHzDENy |
| SOLANA_COMMITMENT | Solana commitment level | confirmed |
| JWT_SECRET | Secret for JWT tokens | your_jwt_secret_key_change_this_in_production |
| JWT_EXPIRATION | JWT token expiration in seconds | 86400 (24 hours) |
| RATE_LIMIT_REQUESTS | Rate limit requests per duration | 100 |
| RATE_LIMIT_DURATION | Rate limit duration in seconds | 60 |

## Testing

Run the tests using:

```bash
cargo test
```

## License

This project is licensed under the MIT License. 