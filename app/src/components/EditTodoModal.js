import React, { useState } from 'react';

const EditTodoModal = ({ todo, onClose, onSubmit, loading }) => {
  const [description, setDescription] = useState(todo.description);

  const handleSubmit = (e) => {
    e.preventDefault();
    if (!description.trim()) return;
    onSubmit(description);
  };

  return (
    <div className="modal-overlay">
      <div className="modal">
        <h2>Edit Todo</h2>
        <form onSubmit={handleSubmit}>
          <input
            type="text"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder="What needs to be done?"
            disabled={loading}
            required
          />
          <div className="modal-actions">
            <button type="button" onClick={onClose} disabled={loading}>
              Cancel
            </button>
            <button type="submit" disabled={loading}>
              {loading ? 'Saving...' : 'Save'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default EditTodoModal; 