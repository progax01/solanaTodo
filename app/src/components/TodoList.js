import React from 'react';

const TodoList = ({ todos, onToggleComplete, onEditTodo, onDeleteTodo, loading }) => {
  // Format date from timestamp
  const formatDate = (timestamp) => {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
  };

  // If no todos, show empty state
  if (todos.length === 0 && !loading) {
    return (
      <div className="todo-list">
        <div className="empty-state">
          <h2>No Todos Yet</h2>
          <p>Create your first todo item above!</p>
        </div>
      </div>
    );
  }

  return (
    <div className="todo-list">
      <div className="todo-list-header">
        <div>Your Todo List</div>
        <div>{todos.length} item{todos.length !== 1 ? 's' : ''}</div>
      </div>
      
      {todos.map((todo) => (
        <div key={todo.account.id.toString()} className="todo-item">
          <input
            type="checkbox"
            className="todo-checkbox"
            checked={todo.account.completed}
            onChange={() => onToggleComplete(todo, !todo.account.completed)}
            disabled={loading}
          />
          
          <div className="todo-content">
            <p className={`todo-description ${todo.account.completed ? 'completed' : ''}`}>
              {todo.account.description}
            </p>
            <div className="todo-metadata">
              Due: {formatDate(todo.account.dueDate)} | ID: {todo.account.id.toString()}
            </div>
          </div>
          
          <div className="todo-actions">
            <button
              onClick={() => onEditTodo(todo)}
              className="edit-button"
              disabled={loading}
              title="Edit"
            >
              Edit
            </button>
            <button
              onClick={() => onDeleteTodo(todo)}
              className="delete-button"
              disabled={loading}
              title="Delete"
            >
              Delete
            </button>
          </div>
        </div>
      ))}
    </div>
  );
};

export default TodoList; 