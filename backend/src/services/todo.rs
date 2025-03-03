use crate::{
    error::AppError,
    models::todo::{CreateTodoRequest, Todo, TodoResponse, UpdateTodoRequest},
    services::solana::SolanaService,
};

#[derive(Clone)]
pub struct TodoService {
    solana_service: SolanaService,
}

impl TodoService {
    pub fn new(solana_service: SolanaService) -> Self {
        Self { solana_service }
    }

    // Get all todos for a wallet
    pub async fn get_todos(&self, public_key: &str) -> Result<Vec<TodoResponse>, AppError> {
        let todos = self.solana_service.get_todos(public_key).await?;
        let todo_responses = todos.into_iter().map(TodoResponse::from).collect();
        
        Ok(todo_responses)
    }

    // Create a new todo
    pub async fn create_todo(
        &self,
        public_key: &str,
        todo: CreateTodoRequest,
    ) -> Result<TodoResponse, AppError> {
        let new_todo = self.solana_service.create_todo(public_key, todo).await?;
        let todo_response = TodoResponse::from(new_todo);
        
        Ok(todo_response)
    }

    // Update a todo
    pub async fn update_todo(
        &self,
        public_key: &str,
        todo_id: u64,
        update: UpdateTodoRequest,
    ) -> Result<TodoResponse, AppError> {
        let updated_todo = self.solana_service.update_todo(public_key, todo_id, update).await?;
        let todo_response = TodoResponse::from(updated_todo);
        
        Ok(todo_response)
    }

    // Delete a todo
    pub async fn delete_todo(&self, public_key: &str, todo_id: u64) -> Result<(), AppError> {
        self.solana_service.delete_todo(public_key, todo_id).await
    }
} 