import React, { useState } from 'react';

const TodoForm = ({ onCreateTodo, disabled }) => {
  const [description, setDescription] = useState('');
  const [dueDate, setDueDate] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = (e) => {
    e.preventDefault();
    
    // Validate inputs
    if (!description.trim()) {
      setError('Description is required');
      return;
    }
    
    if (description.length > 280) {
      setError('Description must be 280 characters or less');
      return;
    }
    
    if (!dueDate) {
      setError('Due date is required');
      return;
    }
    
    // Parse due date
    const dueDateObj = new Date(dueDate);
    
    // Reset error
    setError('');
    
    // Call parent handler
    onCreateTodo(description, dueDateObj);
    
    // Reset form
    setDescription('');
    setDueDate('');
  };

  return (
    <div className="todo-form">
      <h2>Add New Todo</h2>
      
      {error && (
        <div className="error-message">
          {error}
        </div>
      )}
      
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="description">Description</label>
          <textarea
            id="description"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder="What needs to be done?"
            maxLength={280}
            disabled={disabled}
          />
          <small>{description.length}/280 characters</small>
        </div>
        
        <div className="form-group">
          <label htmlFor="dueDate">Due Date</label>
          <input
            id="dueDate"
            type="datetime-local"
            value={dueDate}
            onChange={(e) => setDueDate(e.target.value)}
            disabled={disabled}
          />
        </div>
        
        <button
          type="submit"
          className="button"
          disabled={disabled}
        >
          Add Todo
        </button>
      </form>
    </div>
  );
};

export default TodoForm; 