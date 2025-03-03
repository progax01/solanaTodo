import React, { useState } from 'react';

const EditTodoModal = ({ todo, onClose, onSave, disabled }) => {
  const [description, setDescription] = useState(todo.description);
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
    
    // Reset error
    setError('');
    
    // Call parent handler
    onSave(description);
  };

  return (
    <div className="modal-backdrop">
      <div className="modal-content">
        <div className="modal-header">
          <h2>Edit Todo</h2>
        </div>
        
        {error && (
          <div className="error-message">
            {error}
          </div>
        )}
        
        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="edit-description">Description</label>
            <textarea
              id="edit-description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="What needs to be done?"
              maxLength={280}
              disabled={disabled}
            />
            <small>{description.length}/280 characters</small>
          </div>
          
          <div className="modal-footer">
            <button
              type="button"
              className="button cancel-button"
              onClick={onClose}
              disabled={disabled}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="button"
              disabled={disabled}
            >
              Save Changes
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default EditTodoModal; 