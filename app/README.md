# Solana Todo App Frontend

This is the frontend for the Solana Todo application, a decentralized todo list built on the Solana blockchain. It follows a design pattern where transactions are prepared on the backend, signed by the user's wallet on the frontend, and then submitted back to the backend for processing.

## Features

- Connect with Solana wallets (Phantom, Solflare)
- Authenticate with digital signatures
- Create, read, update, and delete todos
- Sign transactions with your wallet
- Modern, responsive UI

## Prerequisites

- Node.js (v14 or higher)
- npm or yarn
- Solana wallet (Phantom, Solflare, etc.)
- Backend server running (see backend README)

## Setup and Installation

1. Install dependencies:

```bash
npm install
```

2. Make sure the backend is running on `http://localhost:8080`. If it's running on a different URL, update the `API_URL` constant in `src/components/TodoApp.js`.

3. Start the development server:

```bash
npm start
```

4. Open your browser and navigate to `http://localhost:3000`

## Transaction Flow

The application follows this transaction flow:

1. **Transaction Construction**:
   - The backend prepares the transaction with all required instructions and fee payer details.

2. **Sending to Frontend**:
   - The backend sends the constructed transaction to the frontend.

3. **User Signature via Wallet Adapter**:
   - On the frontend, the wallet adapter prompts the user to sign the transaction.

4. **Submission to Network**:
   - The signed transaction is sent back to the backend for submission to the Solana network.

## Folder Structure

```
src/
├── components/            # React components
│   ├── TodoApp.js         # Main application component
│   ├── TodoForm.js        # Form for creating todos
│   ├── TodoList.js        # List of todos
│   └── EditTodoModal.js   # Modal for editing todos
├── idl/                   # Solana program interface definitions
│   └── solana_todo.json   # IDL for the Solana Todo program
├── App.js                 # Main App component with wallet adapter setup
├── App.css                # Application styling
├── index.js               # Entry point
└── index.css              # Global styles
```

## Development

To build for production:

```bash
npm run build
```

## Notes

- This application uses the Solana wallet adapter for wallet connections
- Authentication is handled via message signing
- Transactions are prepared on the backend to ensure proper validation and security
- The backend is responsible for submitting signed transactions to the Solana network 