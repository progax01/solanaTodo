# Solana Todo List

A decentralized todo list application built on the Solana blockchain using the Anchor framework.

## Features

- Create todo items with description and due date
- Mark todos as complete/incomplete
- Update todo descriptions
- Delete todo items
- User-specific todo lists (each wallet has its own list)
- Proper access control (only owners can modify their todos)

## Project Structure

- `programs/solana_todo/src/lib.rs`: The Solana program (smart contract) written in Rust using the Anchor framework
- `tests/solana_todo.ts`: Tests for the Solana program
- `app/`: Frontend application built with React and the Solana/Anchor client libraries

## Technical Specifications

- Built using the Anchor framework for Solana
- Implements proper access control (only owners can modify their todos)
- Includes error handling and input validation
- Unit tests for program functionality

## Getting Started

### Prerequisites

- Node.js and npm/yarn
- Rust and Cargo
- Solana CLI tools
- Anchor CLI

### Installation

1. Clone the repository
2. Install dependencies:

```bash
cd solana_todo
yarn install
```

### Building and Deploying the Solana Program

```bash
anchor build
anchor deploy
```

### Running the Tests

```bash
anchor test
```

### Running the Frontend Application

```bash
cd app
yarn install
yarn start
```

## Account Structure

### UserProfile Account

Stores information about a user's todo list:

- `authority`: User's public key
- `todo_count`: Number of active todos
- `last_todo_id`: Last assigned todo ID

### TodoItem Account

Stores information about a single todo item:

- `id`: Unique task ID
- `description`: Task description (max 280 characters)
- `completed`: Completion status
- `due_date`: Due date timestamp
- `owner`: Owner's public key
- `authority`: Authority to modify the todo

## Instructions

The program implements the following instructions:

1. `initialize_user`: Creates a new user profile
2. `create_todo`: Creates a new todo item
3. `update_todo_status`: Marks a todo as complete or incomplete
4. `update_description`: Updates a todo's description
5. `delete_todo`: Deletes a todo item

## License

This project is licensed under the MIT License. 