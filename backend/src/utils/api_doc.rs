use chrono::{DateTime, Utc};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::models::{auth::{AuthRequest, AuthResponse}, todo::{CreateTodoRequest, TodoResponse, UpdateTodoRequest}};
use crate::controllers::{auth, todo};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::authenticate,
        todo::get_todos,
        todo::create_todo,
        todo::update_todo,
        todo::delete_todo
    ),
    components(
        schemas(
            AuthRequest,
            AuthResponse,
            CreateTodoRequest,
            UpdateTodoRequest,
            TodoResponse,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "todos", description = "Todo management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        )
    }
} 