use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PreparedTransaction {
    #[schema(example = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDyjtJ6Jf8zd9P/YoVvMM9Yg9n/vYMzjBFtKyI5IVwBGNBxiqYhI0UXeG9LrYPk4p8lZCLXKJTa6nQ61KJ1gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA4O5d4OTMrMvbCpVv5maDe82SnNxl6OYOHAMBIwXxF0PS7UOi9/tQYZcG9zV9k5OTcUXJIcI+iRGhcGQly3yZAgAAAAAHYUJnTxU6m3Fck70YwVzLpNcbJ0yKVpzQzLcR7y9ZVQEDAwABAEoAAAAAAAAAjbv2dF80rSTn3xTrH9Y1XRdgFxvLhd+jN2gDBQD/GfwMjinlkDl90P7xO3YKBp9BQkM/fqGcJ5aoVaA8tzoBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")]
    pub serialized_transaction: String,

    #[schema(example = "create_todo")]
    pub transaction_type: String,

    #[schema(example = "{\"todo_id\":1,\"description\":\"Buy groceries\"}")]
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SignedTransaction {
    #[schema(example = "3WZ4sBJm5eGzRfRXLCuUktaE7xMW25dveKfpGxCCkyvMfPQiJKgf1nuFCvhKTQUCHYKGEE6KuQnUijBTKMhNcWd7")]
    pub signature: String,
    
    #[schema(example = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDyjtJ6Jf8zd9P/YoVvMM9YoFnVvYMzjBFtKyI5IVwBGNBxiqYhI0UXeG9LrYPk4p8lZCLXKJTa6nQ61KJ1gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA4O5d4OTMrMvbCpVv5maDe82SnNxl6OYOHAMBIwXxF0PS7UOi9/tQYZcG9zV9k5OTcUXJIcI+iRGhcGQly3yZAgAAAAAHYUJnTxU6m3Fck70YwVzLpNcbJ0yKVpzQzLcR7y9ZVQEDAwABAEoAAAAAAAAAjbv2dF80rSTn3xTrH9Y1XRdgFxvLhd+jN2gDBQD/GfwMjinlkDl90P7xO3YKBp9BQkM/fqGcJ5aoVaA8tzoBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")]
    pub serialized_transaction: String,
}

// Request for preparing transactions
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateTodoTransactionRequest {
    #[schema(example = "Complete the Solana project")]
    pub description: String,
    
    #[schema(example = "1625097600")]
    pub due_date: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateTodoTransactionRequest {
    #[schema(example = "Complete the Solana project updated")]
    pub description: Option<String>,
    
    #[schema(example = "true")]
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DeleteTodoTransactionRequest {
    #[schema(example = "1")]
    pub todo_id: u64,
} 