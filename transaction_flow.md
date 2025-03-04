# Solana Todo dApp Transaction Flow

## Overview

The Solana Todo dApp implements a secure transaction flow that leverages both the backend and frontend to create a smooth user experience while maintaining security. This document details how transactions are handled in the application.

## Transaction Flow Diagram

```
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│               │  1   │               │  3   │               │
│    Frontend   │─────▶│    Backend    │◀─────│     Solana    │
│  (React.js)   │◀─────│  (Actix-Web)  │─────▶│    Network    │
│               │  2   │               │  4   │               │
└───────────────┘      └───────────────┘      └───────────────┘
        │                      ▲                      ▲
        │                      │                      │
        │                      │                      │
        ▼                      │                      │
┌───────────────┐              │                      │
│               │              │                      │
│    Wallet     │              │                      │
│    Adapter    │              │                      │
│               │              │                      │
└───────────────┘              │                      │
        │                      │                      │
        └──────────────────────┴──────────────────────┘
```

## Step-by-Step Transaction Flow

### 1. Transaction Construction (Frontend → Backend)

1. User initiates an action (create/update/delete todo) in the frontend
2. Frontend makes a request to the backend's transaction preparation endpoint:
   - `POST /api/transactions/prepare/create` for creating todos
   - `POST /api/transactions/prepare/update/{id}` for updating todos 
   - `POST /api/transactions/prepare/delete` for deleting todos

### 2. Transaction Preparation (Backend → Frontend)

1. Backend constructs the appropriate Solana transaction:
   - Creates the necessary instruction data
   - Sets up the required account metas
   - Generates an unsigned transaction
2. Backend serializes the transaction and returns it to the frontend as a `PreparedTransaction` object
3. Frontend receives the transaction data

### 3. Transaction Signing (Frontend → Wallet → Backend)

1. Frontend deserializes the transaction
2. Frontend requests the wallet adapter to sign the transaction
3. Wallet prompts the user for approval
4. User signs the transaction with their wallet
5. Frontend sends the signed transaction to the backend's submit endpoint:
   - `POST /api/transactions/submit`

### 4. Transaction Submission (Backend → Solana Network)

1. Backend deserializes the signed transaction
2. Backend submits the transaction to the Solana network
3. Backend waits for confirmation of the transaction
4. Backend returns the transaction signature to the frontend
5. Frontend updates the UI based on the response

## API Endpoints

### Authentication

- **POST /api/auth**
  - Authenticates a user via wallet signature
  - Request body:
    ```json
    {
      "public_key": "HXtBm8XZbxaTt41uqaKhwUAa6Z1aPyvJdsZVENiWsetg",
      "signature": "3AuheKDvzxG6QM4gQHPNFTQ5wGz3aEZnECJK5Lp1e5orTAsyUNZKrGZq25e3XPQZiLgVj7LNzjwERFvxdL4Zx54M",
      "timestamp": 1625097600
    }
    ```
  - Response:
    ```json
    {
      "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
      "expires_in": 86400,
      "public_key": "HXtBm8XZbxaTt41uqaKhwUAa6Z1aPyvJdsZVENiWsetg"
    }
    ```

### Todo Management

- **GET /api/todos**
  - Retrieves all todos for the authenticated user
  - Requires Authorization header: `Bearer <token>`
  - Response: Array of Todo objects

- **POST /api/todos**
  - Creates a new todo (direct method)
  - Requires Authorization header: `Bearer <token>`
  - Request body:
    ```json
    {
      "description": "Complete the Solana project",
      "due_date": 1625097600
    }
    ```
  - Response: Created Todo object

- **PUT /api/todos/{id}**
  - Updates an existing todo (direct method)
  - Requires Authorization header: `Bearer <token>`
  - Request body:
    ```json
    {
      "description": "Updated description",
      "completed": true
    }
    ```
  - Response: Updated Todo object

- **DELETE /api/todos/{id}**
  - Deletes a todo (direct method)
  - Requires Authorization header: `Bearer <token>`
  - Response: Success message

### Transaction Preparation

- **POST /api/transactions/prepare/create**
  - Prepares a transaction for creating a todo
  - Requires Authorization header: `Bearer <token>`
  - Request body:
    ```json
    {
      "description": "Complete the Solana project",
      "due_date": 1625097600
    }
    ```
  - Response:
    ```json
    {
      "serialized_transaction": "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDyjtJ6Jf8zd9P...",
      "transaction_type": "create_todo",
      "metadata": "{\"description\":\"Complete the Solana project\",\"due_date\":1625097600}"
    }
    ```

- **POST /api/transactions/prepare/update/{id}**
  - Prepares a transaction for updating a todo
  - Requires Authorization header: `Bearer <token>`
  - Request body:
    ```json
    {
      "description": "Updated description",
      "completed": true
    }
    ```
  - Response:
    ```json
    {
      "serialized_transaction": "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDyjtJ6Jf8zd9P...",
      "transaction_type": "update_todo",
      "metadata": "{\"todo_id\":1,\"description\":\"Updated description\",\"completed\":true}"
    }
    ```

- **POST /api/transactions/prepare/delete**
  - Prepares a transaction for deleting a todo
  - Requires Authorization header: `Bearer <token>`
  - Request body:
    ```json
    {
      "todo_id": 1
    }
    ```
  - Response:
    ```json
    {
      "serialized_transaction": "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDyjtJ6Jf8zd9P...",
      "transaction_type": "delete_todo",
      "metadata": "{\"todo_id\":1}"
    }
    ```

### Transaction Submission

- **POST /api/transactions/submit**
  - Submits a signed transaction to the Solana network
  - Requires Authorization header: `Bearer <token>`
  - Request body:
    ```json
    {
      "signature": "3WZ4sBJm5eGzRfRXLCuUktaE7xMW25dveKfpGxCCkyvMfPQiJKgf1nuFCvhKTQUCHYKGEE6KuQnUijBTKMhNcWd7",
      "serialized_transaction": "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDyjtJ6Jf8zd9P..."
    }
    ```
  - Response:
    ```json
    {
      "signature": "3WZ4sBJm5eGzRfRXLCuUktaE7xMW25dveKfpGxCCkyvMfPQiJKgf1nuFCvhKTQUCHYKGEE6KuQnUijBTKMhNcWd7"
    }
    ```

## Benefits of This Transaction Flow

1. **Security**: Private keys never leave the user's wallet
2. **Usability**: Complex Solana transactions are abstracted away from the user
3. **Flexibility**: The backend can validate transactions before submission
4. **Efficiency**: The backend can batch multiple instructions into a single transaction
5. **Monitoring**: The backend can track transaction status and handle retries
6. **Cost Management**: The backend can ensure transactions are properly funded with fees 