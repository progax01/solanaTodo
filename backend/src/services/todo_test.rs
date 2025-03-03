#[cfg(test)]
mod tests {
    use crate::{
        error::AppResult,
        models::todo::{CreateTodoRequest, Todo, UpdateTodoRequest},
        services::solana::SolanaService,
        services::todo::TodoService,
    };
    use mockall::predicate::*;
    use mockall::*;

    // Mock the SolanaService for testing
    mock! {
        SolanaServiceMock {}

        impl Clone for SolanaServiceMock {
            fn clone(&self) -> Self;
        }

        #[async_trait::async_trait]
        impl SolanaService {
            fn new() -> Result<Self, crate::error::AppError>;
            async fn get_todos(&self, public_key: &str) -> Result<Vec<Todo>, crate::error::AppError>;
            async fn create_todo(&self, public_key: &str, todo: CreateTodoRequest) -> Result<Todo, crate::error::AppError>;
            async fn update_todo(&self, public_key: &str, todo_id: u64, update: UpdateTodoRequest) -> Result<Todo, crate::error::AppError>;
            async fn delete_todo(&self, public_key: &str, todo_id: u64) -> Result<(), crate::error::AppError>;
        }
    }

    #[actix_rt::test]
    async fn test_get_todos() {
        // Setup mock
        let mut mock = MockSolanaServiceMock::new();
        mock.expect_get_todos()
            .with(eq("test_public_key"))
            .returning(|_| {
                Ok(vec![
                    Todo {
                        id: 1,
                        description: "Test todo".to_string(),
                        completed: false,
                        due_date: 1625097600,
                        owner: "test_public_key".to_string(),
                    },
                ])
            });

        // Create service with mock
        let todo_service = TodoService::new(mock);

        // Call the service
        let result = todo_service.get_todos("test_public_key").await;

        // Assert
        assert!(result.is_ok());
        let todos = result.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].id, 1);
        assert_eq!(todos[0].description, "Test todo");
        assert_eq!(todos[0].completed, false);
    }

    #[actix_rt::test]
    async fn test_create_todo() {
        // Setup mock
        let mut mock = MockSolanaServiceMock::new();
        mock.expect_create_todo()
            .with(eq("test_public_key"), function(|req: &CreateTodoRequest| {
                req.description == "New todo" && req.due_date == 1625097600
            }))
            .returning(|_, _| {
                Ok(Todo {
                    id: 1,
                    description: "New todo".to_string(),
                    completed: false,
                    due_date: 1625097600,
                    owner: "test_public_key".to_string(),
                })
            });

        // Create service with mock
        let todo_service = TodoService::new(mock);

        // Create request
        let request = CreateTodoRequest {
            description: "New todo".to_string(),
            due_date: 1625097600,
        };

        // Call the service
        let result = todo_service.create_todo("test_public_key", request).await;

        // Assert
        assert!(result.is_ok());
        let todo = result.unwrap();
        assert_eq!(todo.id, 1);
        assert_eq!(todo.description, "New todo");
        assert_eq!(todo.completed, false);
    }

    #[actix_rt::test]
    async fn test_update_todo() {
        // Setup mock
        let mut mock = MockSolanaServiceMock::new();
        mock.expect_update_todo()
            .with(eq("test_public_key"), eq(1), function(|req: &UpdateTodoRequest| {
                req.description.as_ref().unwrap() == "Updated todo" && req.completed.unwrap() == true
            }))
            .returning(|_, _, _| {
                Ok(Todo {
                    id: 1,
                    description: "Updated todo".to_string(),
                    completed: true,
                    due_date: 1625097600,
                    owner: "test_public_key".to_string(),
                })
            });

        // Create service with mock
        let todo_service = TodoService::new(mock);

        // Create request
        let request = UpdateTodoRequest {
            description: Some("Updated todo".to_string()),
            completed: Some(true),
        };

        // Call the service
        let result = todo_service.update_todo("test_public_key", 1, request).await;

        // Assert
        assert!(result.is_ok());
        let todo = result.unwrap();
        assert_eq!(todo.id, 1);
        assert_eq!(todo.description, "Updated todo");
        assert_eq!(todo.completed, true);
    }

    #[actix_rt::test]
    async fn test_delete_todo() {
        // Setup mock
        let mut mock = MockSolanaServiceMock::new();
        mock.expect_delete_todo()
            .with(eq("test_public_key"), eq(1))
            .returning(|_, _| Ok(()));

        // Create service with mock
        let todo_service = TodoService::new(mock);

        // Call the service
        let result = todo_service.delete_todo("test_public_key", 1).await;

        // Assert
        assert!(result.is_ok());
    }
} 