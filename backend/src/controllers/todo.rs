use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse};
use log::{error, info};
use utoipa::OpenApi;

use crate::{
    error::AppResult,
    models::{
        auth::AuthToken,
        todo::{CreateTodoRequest, TodoResponse, UpdateTodoRequest},
    },
    services::todo::TodoService,
};

#[utoipa::path(
    get,
    path = "/api/todos",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of todos", body = Vec<TodoResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 429, description = "Rate limit exceeded"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("")]
pub async fn get_todos(
    req: HttpRequest,
    todo_service: web::Data<TodoService>,
) -> AppResult<HttpResponse> {
    let extensions = req.extensions();
    let auth_token = extensions.get::<AuthToken>().unwrap();
    
    info!("Getting todos for user: {}", auth_token.public_key);
    
    let todos = todo_service.get_todos(&auth_token.public_key).await?;
    
    Ok(HttpResponse::Ok().json(todos))
}

#[utoipa::path(
    post,
    path = "/api/todos",
    request_body = CreateTodoRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Todo created successfully", body = TodoResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 429, description = "Rate limit exceeded"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("")]
pub async fn create_todo(
    req: HttpRequest,
    todo_service: web::Data<TodoService>,
    todo_request: web::Json<CreateTodoRequest>,
) -> AppResult<HttpResponse> {
    let extensions = req.extensions();
    let auth_token = extensions.get::<AuthToken>().unwrap();
    
    info!("Creating todo for user: {}", auth_token.public_key);
    
    let todo = todo_service
        .create_todo(&auth_token.public_key, todo_request.into_inner())
        .await?;
    
    Ok(HttpResponse::Created().json(todo))
}

#[utoipa::path(
    put,
    path = "/api/todos/{id}",
    params(
        ("id" = u64, Path, description = "Todo ID")
    ),
    request_body = UpdateTodoRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Todo updated successfully", body = TodoResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Todo not found"),
        (status = 429, description = "Rate limit exceeded"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/{id}")]
pub async fn update_todo(
    req: HttpRequest,
    path: web::Path<u64>,
    todo_service: web::Data<TodoService>,
    update_request: web::Json<UpdateTodoRequest>,
) -> AppResult<HttpResponse> {
    let extensions = req.extensions();
    let auth_token = extensions.get::<AuthToken>().unwrap();
    let todo_id = path.into_inner();
    
    info!("Updating todo {} for user: {}", todo_id, auth_token.public_key);
    
    let todo = todo_service
        .update_todo(&auth_token.public_key, todo_id, update_request.into_inner())
        .await?;
    
    Ok(HttpResponse::Ok().json(todo))
}

#[utoipa::path(
    delete,
    path = "/api/todos/{id}",
    params(
        ("id" = u64, Path, description = "Todo ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Todo deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Todo not found"),
        (status = 429, description = "Rate limit exceeded"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/{id}")]
pub async fn delete_todo(
    req: HttpRequest,
    path: web::Path<u64>,
    todo_service: web::Data<TodoService>,
) -> AppResult<HttpResponse> {
    let extensions = req.extensions();
    let auth_token = extensions.get::<AuthToken>().unwrap();
    let todo_id = path.into_inner();
    
    info!("Deleting todo {} for user: {}", todo_id, auth_token.public_key);
    
    todo_service.delete_todo(&auth_token.public_key, todo_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
} 