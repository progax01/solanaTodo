import React from 'react';

const TodoList = ({ todos, loading, onToggleComplete, onDelete, onEdit }) => {
  if (loading) {
    return <div className="loading-message">Loading todos...</div>;
  }

  if (!todos || todos.length === 0) {
    return <div className="empty-message">No todos yet. Add one above!</div>;
  }

  // Format date from timestamp
  const formatDate = (timestamp) => {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
  };

  return (
    <div>
      <div className="todo-list-header">
        Your Todos ({todos.length})
      </div>
      <ul className="todo-list">
        {todos.map((todo) => (
          <li 
            key={todo.id} 
            className={`todo-item ${todo.completed ? 'todo-completed' : ''}`}
          >
            <input
              type="checkbox"
              className="todo-checkbox"
              checked={todo.completed}
              onChange={() => onToggleComplete(todo, !todo.completed)}
            />
            <div className="todo-content">
              <div className="todo-description">{todo.description}</div>
              <div className="todo-date">Due: {formatDate(todo.due_date)}</div>
            </div>
            <div className="todo-actions">
              <button 
                className="edit-btn"
                onClick={() => onEdit(todo)}
              >
                Edit
              </button>
              <button 
                className="delete-btn"
                onClick={() => onDelete(todo)}
              >
                Delete
              </button>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default TodoList; 