import React, { useState, useEffect } from 'react';
import { useConnection, useAnchorWallet } from '@solana/wallet-adapter-react';
import { PublicKey, SystemProgram, Transaction } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { BN } from 'bn.js';

import TodoForm from './TodoForm';
import TodoList from './TodoList';
import EditTodoModal from './EditTodoModal';
import idl from '../idl/solana_todo.json';

const PROGRAM_ID = new PublicKey(idl.metadata.address);
const API_URL = 'http://localhost:8080'; // Backend API URL

const TodoApp = () => {
  const { connection } = useConnection();
  const wallet = useAnchorWallet();
  
  const [loading, setLoading] = useState(false);
  const [initialized, setInitialized] = useState(false);
  const [todos, setTodos] = useState([]);
  const [userProfile, setUserProfile] = useState(null);
  const [errorMessage, setErrorMessage] = useState('');
  const [authToken, setAuthToken] = useState('');
  
  // Edit modal state
  const [isEditModalOpen, setIsEditModalOpen] = useState(false);
  const [editingTodo, setEditingTodo] = useState(null);

  // Authenticate with the backend
  const authenticate = async () => {
    if (!wallet) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      // Create a timestamp
      const timestamp = Math.floor(Date.now() / 1000);
      
      // Create the message to sign
      const message = `Sign in to Solana Todo App: ${timestamp}`;
      
      // Sign the message with the wallet
      const messageBytes = new TextEncoder().encode(message);
      const signature = await wallet.adapter.signMessage(messageBytes);
      const signatureBase58 = Buffer.from(signature).toString('base64');
      
      // Send to backend
      const response = await fetch(`${API_URL}/api/auth`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          public_key: wallet.publicKey.toString(),
          signature: signatureBase58,
          timestamp,
        }),
      });
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Authentication failed');
      }
      
      const data = await response.json();
      setAuthToken(data.token);
      
      // Load todos after authentication
      loadTodos(data.token);
      
    } catch (error) {
      console.error('Authentication error:', error);
      setErrorMessage(`Authentication failed: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Load todos from the backend
  const loadTodos = async (token) => {
    if (!token && !authToken) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      const response = await fetch(`${API_URL}/api/todos`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token || authToken}`,
        },
      });
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Failed to load todos');
      }
      
      const data = await response.json();
      setTodos(data);
      
    } catch (error) {
      console.error('Error loading todos:', error);
      setErrorMessage(`Failed to load todos: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Create a todo
  const createTodo = async (description, dueDate) => {
    if (!wallet || !authToken) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      // 1. Prepare the transaction on the backend
      const prepareResponse = await fetch(`${API_URL}/api/transactions/prepare/create`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authToken}`,
        },
        body: JSON.stringify({
          description,
          due_date: Math.floor(dueDate.getTime() / 1000),
        }),
      });
      
      if (!prepareResponse.ok) {
        const errorData = await prepareResponse.json();
        throw new Error(errorData.message || 'Failed to prepare transaction');
      }
      
      const preparedTx = await prepareResponse.json();
      
      // 2. Decode the serialized transaction
      const serializedTransaction = Buffer.from(preparedTx.serialized_transaction, 'base64');
      const transaction = Transaction.from(serializedTransaction);
      
      // 3. Sign the transaction with the wallet
      transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
      transaction.feePayer = wallet.publicKey;
      const signedTransaction = await wallet.signTransaction(transaction);
      const serializedSignedTransaction = signedTransaction.serialize().toString('base64');
      
      // 4. Send the signed transaction back to the backend
      const submitResponse = await fetch(`${API_URL}/api/transactions/submit`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authToken}`,
        },
        body: JSON.stringify({
          signature: transaction.signatures[0].signature.toString('base64'),
          serialized_transaction: serializedSignedTransaction,
        }),
      });
      
      if (!submitResponse.ok) {
        const errorData = await submitResponse.json();
        throw new Error(errorData.message || 'Failed to submit transaction');
      }
      
      // 5. Reload todos to refresh the list
      await loadTodos();
      
    } catch (error) {
      console.error('Error creating todo:', error);
      setErrorMessage(`Failed to create todo: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Update todo status
  const updateTodoStatus = async (todo, completed) => {
    if (!wallet || !authToken) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      // 1. Prepare the transaction on the backend
      const prepareResponse = await fetch(`${API_URL}/api/transactions/prepare/update/${todo.id}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authToken}`,
        },
        body: JSON.stringify({
          completed,
        }),
      });
      
      if (!prepareResponse.ok) {
        const errorData = await prepareResponse.json();
        throw new Error(errorData.message || 'Failed to prepare transaction');
      }
      
      const preparedTx = await prepareResponse.json();
      
      // 2. Decode the serialized transaction
      const serializedTransaction = Buffer.from(preparedTx.serialized_transaction, 'base64');
      const transaction = Transaction.from(serializedTransaction);
      
      // 3. Sign the transaction with the wallet
      transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
      transaction.feePayer = wallet.publicKey;
      const signedTransaction = await wallet.signTransaction(transaction);
      const serializedSignedTransaction = signedTransaction.serialize().toString('base64');
      
      // 4. Send the signed transaction back to the backend
      const submitResponse = await fetch(`${API_URL}/api/transactions/submit`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authToken}`,
        },
        body: JSON.stringify({
          signature: transaction.signatures[0].signature.toString('base64'),
          serialized_transaction: serializedSignedTransaction,
        }),
      });
      
      if (!submitResponse.ok) {
        const errorData = await submitResponse.json();
        throw new Error(errorData.message || 'Failed to submit transaction');
      }
      
      // 5. Reload todos to refresh the list
      await loadTodos();
      
    } catch (error) {
      console.error('Error updating todo:', error);
      setErrorMessage(`Failed to update todo: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Update todo description
  const updateTodoDescription = async (todo, description) => {
    if (!wallet || !authToken) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      // 1. Prepare the transaction on the backend
      const prepareResponse = await fetch(`${API_URL}/api/transactions/prepare/update/${todo.id}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authToken}`,
        },
        body: JSON.stringify({
          description,
        }),
      });
      
      if (!prepareResponse.ok) {
        const errorData = await prepareResponse.json();
        throw new Error(errorData.message || 'Failed to prepare transaction');
      }
      
      const preparedTx = await prepareResponse.json();
      
      // 2. Decode the serialized transaction
      const serializedTransaction = Buffer.from(preparedTx.serialized_transaction, 'base64');
      const transaction = Transaction.from(serializedTransaction);
      
      // 3. Sign the transaction with the wallet
      transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
      transaction.feePayer = wallet.publicKey;
      const signedTransaction = await wallet.signTransaction(transaction);
      const serializedSignedTransaction = signedTransaction.serialize().toString('base64');
      
      // 4. Send the signed transaction back to the backend
      const submitResponse = await fetch(`${API_URL}/api/transactions/submit`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authToken}`,
        },
        body: JSON.stringify({
          signature: transaction.signatures[0].signature.toString('base64'),
          serialized_transaction: serializedSignedTransaction,
        }),
      });
      
      if (!submitResponse.ok) {
        const errorData = await submitResponse.json();
        throw new Error(errorData.message || 'Failed to submit transaction');
      }
      
      // 5. Reload todos to refresh the list
      await loadTodos();
      closeEditModal();
      
    } catch (error) {
      console.error('Error updating todo:', error);
      setErrorMessage(`Failed to update todo: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Delete a todo
  const deleteTodo = async (todo) => {
    if (!wallet || !authToken) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      // 1. Prepare the transaction on the backend
      const prepareResponse = await fetch(`${API_URL}/api/transactions/prepare/delete`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authToken}`,
        },
        body: JSON.stringify({
          todo_id: todo.id,
        }),
      });
      
      if (!prepareResponse.ok) {
        const errorData = await prepareResponse.json();
        throw new Error(errorData.message || 'Failed to prepare transaction');
      }
      
      const preparedTx = await prepareResponse.json();
      
      // 2. Decode the serialized transaction
      const serializedTransaction = Buffer.from(preparedTx.serialized_transaction, 'base64');
      const transaction = Transaction.from(serializedTransaction);
      
      // 3. Sign the transaction with the wallet
      transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
      transaction.feePayer = wallet.publicKey;
      const signedTransaction = await wallet.signTransaction(transaction);
      const serializedSignedTransaction = signedTransaction.serialize().toString('base64');
      
      // 4. Send the signed transaction back to the backend
      const submitResponse = await fetch(`${API_URL}/api/transactions/submit`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authToken}`,
        },
        body: JSON.stringify({
          signature: transaction.signatures[0].signature.toString('base64'),
          serialized_transaction: serializedSignedTransaction,
        }),
      });
      
      if (!submitResponse.ok) {
        const errorData = await submitResponse.json();
        throw new Error(errorData.message || 'Failed to submit transaction');
      }
      
      // 5. Reload todos to refresh the list
      await loadTodos();
      
    } catch (error) {
      console.error('Error deleting todo:', error);
      setErrorMessage(`Failed to delete todo: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Modal control
  const openEditModal = (todo) => {
    setEditingTodo(todo);
    setIsEditModalOpen(true);
  };

  const closeEditModal = () => {
    setIsEditModalOpen(false);
    setEditingTodo(null);
  };

  // Effect to authenticate when wallet changes
  useEffect(() => {
    if (wallet) {
      authenticate();
    } else {
      setTodos([]);
      setAuthToken('');
    }
  }, [wallet]);

  if (!wallet) {
    return (
      <div className="todo-app">
        <div className="wallet-message">
          <p>Please connect your wallet to use the Solana Todo App</p>
        </div>
      </div>
    );
  }

  return (
    <div className="todo-app">
      {errorMessage && (
        <div className="error-message">
          <p>{errorMessage}</p>
          <button onClick={() => setErrorMessage('')}>Dismiss</button>
        </div>
      )}
      
      <div className="wallet-info">
        <p>Connected: {wallet.publicKey.toString()}</p>
        </div>
      
      <TodoForm onSubmit={createTodo} loading={loading} />
      
      <TodoList 
        todos={todos}
        loading={loading}
        onToggleComplete={updateTodoStatus}
        onDelete={deleteTodo}
        onEdit={openEditModal}
      />
      
      {isEditModalOpen && (
        <EditTodoModal
          todo={editingTodo}
          onClose={closeEditModal}
          onSubmit={(description) => updateTodoDescription(editingTodo, description)}
          loading={loading}
        />
      )}
    </div>
  );
};

export default TodoApp; 