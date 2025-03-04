use chrono::{DateTime, Utc};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::models::{
    auth::{AuthRequest, AuthResponse}, 
    todo::{CreateTodoRequest, TodoResponse, UpdateTodoRequest},
    transaction::{PreparedTransaction, SignedTransaction, DeleteTodoTransactionRequest}
};
use crate::controllers::{auth, todo, transaction};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::authenticate,
        todo::get_todos,
        todo::create_todo,
        todo::update_todo,
        todo::delete_todo,
        transaction::prepare_create_transaction,
        transaction::prepare_update_transaction,
        transaction::prepare_delete_transaction,
        transaction::submit_transaction
    ),
    components(
        schemas(
            AuthRequest,
            AuthResponse,
            CreateTodoRequest,
            UpdateTodoRequest,
            TodoResponse,
            PreparedTransaction,
            SignedTransaction,
            DeleteTodoTransactionRequest
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "todos", description = "Todo management endpoints"),
        (name = "transactions", description = "Transaction preparation and submission endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::{Schema, SchemaType};
        
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );
        
        // Add schema for DateTime<Utc>
        let datetime_schema = Schema::Object(
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .format(Some(utoipa::openapi::SchemaFormat::Custom("date-time".to_string())))
                .description(Some("ISO 8601 date and time"))
                .example(Some(serde_json::Value::String("2024-03-03T18:35:30Z".to_string())))
                .build()
        );
        
        components.schemas.insert(
            "DateTime<Utc>".to_string(),
            datetime_schema.into()
        );
    }
} 