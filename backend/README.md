# Solana Todo Backend

This is the backend for the Solana Todo application, a decentralized todo list built on the Solana blockchain. It provides a REST API for interacting with the Solana program.

## Features

- Authentication with Solana wallet signatures
- Todo management (create, read, update, delete)
- Transaction preparation and submission
- Rate limiting and JWT-based authentication
- Swagger documentation

## Prerequisites

- Rust (latest stable version)
- Solana CLI (for local development)
- PostgreSQL (optional, for persistent storage)

## Setup and Installation

1. Clone the repository
2. Navigate to the backend directory
3. Install dependencies:

```bash
cargo build
```

4. Configure the environment variables in `.env` file:

```
# Server configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
RUST_LOG=info

# Solana configuration
SOLANA_RPC_URL=http://localhost:8899
SOLANA_PROGRAM_ID=Ct2N3zw5LFiNj5mJ7hN2c4umze2pAWNjfYqazZHzDENy
SOLANA_COMMITMENT=confirmed

# JWT configuration
JWT_SECRET=fs6TVpXKpMq8oZK4sct2zsUUgvUTE51JZ4gaCvZxU+Y=
JWT_EXPIRATION=86400  # 24 hours in seconds

# Rate limiting
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_DURATION=60  # seconds
```

5. Start the server:

```bash
cargo run
```

The server will be available at `http://localhost:8080`.

## Transaction Flow

The backend is designed to support a secure transaction flow:

1. **Transaction Construction**: The backend prepares transactions with all required instructions and fee payer details
2. **Sending to Frontend**: The constructed transaction is sent to the frontend 
3. **User Signature**: The frontend gets the transaction signed with the user's wallet
4. **Submission to Network**: The signed transaction is sent back to the backend and submitted to the Solana network

For more details, see the [Transaction Flow Documentation](../transaction_flow.md).

## API Endpoints

### Authentication

- **POST /api/auth**: Authenticate with a Solana wallet signature

### Todo Management

- **GET /api/todos**: Get all todos for the authenticated user
- **POST /api/todos**: Create a new todo
- **PUT /api/todos/{id}**: Update a todo
- **DELETE /api/todos/{id}**: Delete a todo

### Transaction Endpoints

- **POST /api/transactions/prepare/create**: Prepare a transaction for creating a todo
- **POST /api/transactions/prepare/update/{id}**: Prepare a transaction for updating a todo
- **POST /api/transactions/prepare/delete**: Prepare a transaction for deleting a todo
- **POST /api/transactions/submit**: Submit a signed transaction

## API Documentation

The API documentation is available at `/api/docs` when the server is running. It provides a Swagger UI interface for exploring and testing the API endpoints.

## Architecture

The backend is built with a modular architecture:

- **Controllers**: Handle HTTP requests and responses
- **Services**: Implement business logic and interact with the Solana blockchain
- **Models**: Define data structures for requests, responses, and domain objects
- **Middlewares**: Implement authentication, rate limiting, and other cross-cutting concerns
- **Utils**: Provide utility functions and helpers
- **Error Handling**: Centralized error handling and standardized error responses

## Development

For local development, you can use the Docker Compose setup to run the backend with a local Solana validator:

```bash
docker-compose up
```

This will start:
- A Solana validator node
- The backend server
- Any other necessary services

## Testing

Run the tests with:

```bash
cargo test
```

Integration tests are available in the `tests` directory and require a running Solana validator.

## Deployment

The backend can be deployed as a Docker container using the provided Dockerfile:

```bash
docker build -t solana-todo-backend .
docker run -p 8080:8080 --env-file .env solana-todo-backend
```

## License

MIT License 