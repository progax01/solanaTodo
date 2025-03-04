use crate::{
    error::AppError,
    models::todo::{CreateTodoRequest, Todo, TodoResponse, UpdateTodoRequest},
    models::transaction::{PreparedTransaction, SignedTransaction},
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
        Ok(TodoResponse::from(new_todo))
    }

    // Update a todo
    pub async fn update_todo(
        &self,
        public_key: &str,
        todo_id: u64,
        update: UpdateTodoRequest,
    ) -> Result<TodoResponse, AppError> {
        let updated_todo = self
            .solana_service
            .update_todo(public_key, todo_id, update)
            .await?;
        Ok(TodoResponse::from(updated_todo))
    }

    // Delete a todo
    pub async fn delete_todo(&self, public_key: &str, todo_id: u64) -> Result<(), AppError> {
        self.solana_service.delete_todo(public_key, todo_id).await
    }
    
    // New methods for transaction preparation
    
    // Prepare a transaction for creating a todo
    pub async fn prepare_create_transaction(
        &self,
        public_key: &str,
        todo: CreateTodoRequest,
    ) -> Result<PreparedTransaction, AppError> {
        self.solana_service.prepare_create_todo_transaction(public_key, todo).await
    }
    
    // Prepare a transaction for updating a todo
    pub async fn prepare_update_transaction(
        &self,
        public_key: &str,
        todo_id: u64,
        update: UpdateTodoRequest,
    ) -> Result<PreparedTransaction, AppError> {
        self.solana_service.prepare_update_todo_transaction(public_key, todo_id, update).await
    }
    
    // Prepare a transaction for deleting a todo
    pub async fn prepare_delete_transaction(
        &self,
        public_key: &str,
        todo_id: u64,
    ) -> Result<PreparedTransaction, AppError> {
        self.solana_service.prepare_delete_todo_transaction(public_key, todo_id).await
    }
    
    // Submit a signed transaction
    pub async fn submit_transaction(
        &self,
        signed_transaction: SignedTransaction,
    ) -> Result<String, AppError> {
        self.solana_service.submit_signed_transaction(signed_transaction).await
    }
} 