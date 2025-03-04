import React, { useState } from 'react';

const TodoForm = ({ onSubmit, loading }) => {
  const [description, setDescription] = useState('');
  const [dueDate, setDueDate] = useState('');

  const handleSubmit = (e) => {
    e.preventDefault();
    if (!description.trim()) return;

    const dueDateObj = dueDate ? new Date(dueDate) : new Date();
    onSubmit(description, dueDateObj);
    setDescription('');
    setDueDate('');
  };

  return (
    <form onSubmit={handleSubmit} className="todo-form">
      <input
        type="text"
        value={description}
        onChange={(e) => setDescription(e.target.value)}
        placeholder="What needs to be done?"
        disabled={loading}
        required
      />
      <input
        type="datetime-local"
        value={dueDate}
        onChange={(e) => setDueDate(e.target.value)}
        disabled={loading}
      />
      <button type="submit" disabled={loading}>
        {loading ? 'Adding...' : 'Add Todo'}
      </button>
    </form>
  );
};

export default TodoForm; 