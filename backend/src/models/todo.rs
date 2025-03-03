use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// Create a newtype wrapper for DateTime<Utc>
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(value_type = String, format = "date-time", example = "2023-07-01T12:00:00Z")]
#[serde(transparent)]
pub struct DateTimeWrapper(DateTime<Utc>);

// Add conversion methods
impl From<DateTime<Utc>> for DateTimeWrapper {
    fn from(dt: DateTime<Utc>) -> Self {
        DateTimeWrapper(dt)
    }
}

impl From<DateTimeWrapper> for DateTime<Utc> {
    fn from(wrapper: DateTimeWrapper) -> Self {
        wrapper.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Todo {
    #[schema(example = "1")]
    pub id: u64,
    
    #[schema(example = "Complete the Solana project")]
    pub description: String,
    
    #[schema(example = "false")]
    pub completed: bool,
    
    #[schema(example = "1625097600")]
    pub due_date: i64,
    
    #[schema(example = "HXtBm8XZbxaTt41uqaKhwUAa6Z1aPyvJdsZVENiWsetg")]
    pub owner: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateTodoRequest {
    #[schema(example = "Complete the Solana project", min_length = 1, max_length = 280)]
    pub description: String,
    
    #[schema(example = "1625097600")]
    pub due_date: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateTodoRequest {
    #[schema(example = "Complete the Solana project updated", min_length = 1, max_length = 280)]
    pub description: Option<String>,
    
    #[schema(example = "true")]
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TodoResponse {
    #[schema(example = "1")]
    pub id: u64,
    
    #[schema(example = "Complete the Solana project")]
    pub description: String,
    
    #[schema(example = "false")]
    pub completed: bool,
    
    #[schema(example = "1625097600")]
    pub due_date: i64,
    
    #[schema(example = "HXtBm8XZbxaTt41uqaKhwUAa6Z1aPyvJdsZVENiWsetg")]
    pub owner: String,
    
    pub created_at: DateTimeWrapper,
    
    pub updated_at: Option<DateTimeWrapper>,
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            description: todo.description,
            completed: todo.completed,
            due_date: todo.due_date,
            owner: todo.owner,
            created_at: DateTimeWrapper(Utc::now()), // Wrap the DateTime
            updated_at: None,
        }
    }
} 