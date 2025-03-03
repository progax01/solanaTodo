import React, { useState, useEffect } from 'react';
import { useConnection, useAnchorWallet } from '@solana/wallet-adapter-react';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { BN } from 'bn.js';

import TodoForm from './TodoForm';
import TodoList from './TodoList';
import EditTodoModal from './EditTodoModal';
import idl from '../idl/solana_todo.json';

const PROGRAM_ID = new PublicKey(idl.metadata.address);

const TodoApp = () => {
  const { connection } = useConnection();
  const wallet = useAnchorWallet();
  
  const [loading, setLoading] = useState(false);
  const [initialized, setInitialized] = useState(false);
  const [todos, setTodos] = useState([]);
  const [userProfile, setUserProfile] = useState(null);
  const [errorMessage, setErrorMessage] = useState('');
  
  // Edit modal state
  const [isEditModalOpen, setIsEditModalOpen] = useState(false);
  const [editingTodo, setEditingTodo] = useState(null);

  // Get our program instance
  const getProgram = () => {
    if (!wallet) return null;
    
    // Create the provider
    const provider = new anchor.AnchorProvider(
      connection,
      wallet,
      { commitment: 'processed' }
    );
    
    // Create the program interface
    const program = new anchor.Program(idl, PROGRAM_ID, provider);
    
    return program;
  };
  
  // Calculate PDA addresses for the user profile
  const getUserProfilePDA = async (authority) => {
    const [userProfilePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("user-profile"), authority.toBuffer()],
      PROGRAM_ID
    );
    return userProfilePDA;
  };

  // Calculate PDA addresses for a todo item
  const getTodoPDA = async (authority, todoId) => {
    const [todoPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("todo"),
        authority.toBuffer(),
        new BN(todoId).toArrayLike(Buffer, "le", 8)
      ],
      PROGRAM_ID
    );
    return todoPDA;
  };

  // Initialize user profile if it doesn't exist
  const initializeUserProfile = async () => {
    if (!wallet) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      const program = getProgram();
      const userProfilePda = await getUserProfilePDA(wallet.publicKey);
      
      // Check if user profile exists
      try {
        const userProfile = await program.account.userProfile.fetch(userProfilePda);
        setUserProfile(userProfile);
        setInitialized(true);
        return userProfile;
      } catch (e) {
        // Profile doesn't exist, let's create it
        console.log("User profile doesn't exist, creating...");
        const tx = await program.methods
          .initializeUser()
          .accounts({
            userProfile: userProfilePda,
            authority: wallet.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .rpc();
          
        console.log("User profile initialized with tx:", tx);
        
        const userProfile = await program.account.userProfile.fetch(userProfilePda);
        setUserProfile(userProfile);
        setInitialized(true);
        return userProfile;
      }
    } catch (error) {
      console.error("Error initializing user profile:", error);
      setErrorMessage(`Error initializing user profile: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Load all todo items
  const loadTodos = async () => {
    if (!wallet || !initialized) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      const program = getProgram();
      
      // Fetch all todos for the current user
      const allTodos = await program.account.todoItem.all([
        {
          memcmp: {
            offset: 8 + 8, // Discriminator (8) + id (8)
            bytes: wallet.publicKey.toBase58()
          }
        }
      ]);
      
      // Sort todos by ID
      const sortedTodos = allTodos.sort((a, b) => 
        a.account.id.toNumber() - b.account.id.toNumber()
      );
      
      setTodos(sortedTodos);
    } catch (error) {
      console.error("Error loading todos:", error);
      setErrorMessage(`Error loading todos: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Create a new todo
  const createTodo = async (description, dueDate) => {
    if (!wallet || !initialized) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      const program = getProgram();
      const userProfilePda = await getUserProfilePDA(wallet.publicKey);
      
      // Refresh user profile to get current lastTodoId
      const userProfile = await program.account.userProfile.fetch(userProfilePda);
      
      // Calculate the next todo ID
      const nextTodoId = userProfile.lastTodoId.toNumber() + 1;
      
      // Calculate todo PDA
      const todoPda = await getTodoPDA(wallet.publicKey, nextTodoId);
      
      // Convert due date to timestamp
      const dueDateTimestamp = Math.floor(dueDate.getTime() / 1000);
      
      // Create the todo
      const tx = await program.methods
        .createTodo(description, new BN(dueDateTimestamp))
        .accounts({
          userProfile: userProfilePda,
          todoAccount: todoPda,
          authority: wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
        
      console.log("Todo created with tx:", tx);
      
      // Reload todos
      await loadTodos();
    } catch (error) {
      console.error("Error creating todo:", error);
      setErrorMessage(`Error creating todo: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Update todo completion status
  const updateTodoStatus = async (todo, completed) => {
    if (!wallet || !initialized) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      const program = getProgram();
      
      // Update the status
      const tx = await program.methods
        .updateTodoStatus(completed)
        .accounts({
          todoAccount: todo.publicKey,
          authority: wallet.publicKey,
        })
        .rpc();
        
      console.log("Todo status updated with tx:", tx);
      
      // Reload todos
      await loadTodos();
    } catch (error) {
      console.error("Error updating todo status:", error);
      setErrorMessage(`Error updating todo status: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Update todo description
  const updateTodoDescription = async (todo, description) => {
    if (!wallet || !initialized) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      const program = getProgram();
      
      // Update the description
      const tx = await program.methods
        .updateDescription(description)
        .accounts({
          todoAccount: todo.publicKey,
          authority: wallet.publicKey,
        })
        .rpc();
        
      console.log("Todo description updated with tx:", tx);
      
      // Close the edit modal
      setIsEditModalOpen(false);
      setEditingTodo(null);
      
      // Reload todos
      await loadTodos();
    } catch (error) {
      console.error("Error updating todo description:", error);
      setErrorMessage(`Error updating description: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Delete a todo
  const deleteTodo = async (todo) => {
    if (!wallet || !initialized) return;
    
    setLoading(true);
    setErrorMessage('');
    
    try {
      const program = getProgram();
      const userProfilePda = await getUserProfilePDA(wallet.publicKey);
      
      // Delete the todo
      const tx = await program.methods
        .deleteTodo()
        .accounts({
          userProfile: userProfilePda,
          todoAccount: todo.publicKey,
          authority: wallet.publicKey,
        })
        .rpc();
        
      console.log("Todo deleted with tx:", tx);
      
      // Reload todos
      await loadTodos();
    } catch (error) {
      console.error("Error deleting todo:", error);
      setErrorMessage(`Error deleting todo: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  // Open edit modal
  const openEditModal = (todo) => {
    setEditingTodo(todo);
    setIsEditModalOpen(true);
  };

  // Close edit modal
  const closeEditModal = () => {
    setIsEditModalOpen(false);
    setEditingTodo(null);
  };

  // Initialize when wallet is connected
  useEffect(() => {
    if (wallet) {
      initializeUserProfile().then(() => {
        loadTodos();
      });
    } else {
      setInitialized(false);
      setUserProfile(null);
      setTodos([]);
    }
  }, [wallet]);

  // If wallet is not connected, show message
  if (!wallet) {
    return (
      <div className="todo-list">
        <div className="empty-state">
          <h2>Connect your wallet</h2>
          <p>Please connect your wallet to use the Todo app</p>
        </div>
      </div>
    );
  }

  return (
    <div>
      {errorMessage && (
        <div className="error-message">
          {errorMessage}
        </div>
      )}
      
      {loading && (
        <div className="loading">
          Loading...
        </div>
      )}
      
      <TodoForm onCreateTodo={createTodo} disabled={loading || !initialized} />
      
      <TodoList 
        todos={todos}
        onToggleComplete={updateTodoStatus}
        onEditTodo={openEditModal}
        onDeleteTodo={deleteTodo}
        loading={loading}
      />
      
      {isEditModalOpen && editingTodo && (
        <EditTodoModal
          todo={editingTodo.account}
          onClose={closeEditModal}
          onSave={(description) => updateTodoDescription(editingTodo, description)}
          disabled={loading}
        />
      )}
    </div>
  );
};

export default TodoApp; 