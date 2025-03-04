use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse};
use log::{error, info};
use utoipa::OpenApi;

use crate::{
    error::AppResult,
    models::{
        auth::AuthToken,
        todo::{CreateTodoRequest, UpdateTodoRequest},
        transaction::{DeleteTodoTransactionRequest, PreparedTransaction, SignedTransaction},
    },
    services::todo::TodoService,
};

#[utoipa::path(
    post,
    path = "/api/transactions/prepare/create",
    request_body = CreateTodoRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Transaction prepared successfully", body = PreparedTransaction),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 429, description = "Rate limit exceeded"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/prepare/create")]
pub async fn prepare_create_transaction(
    req: HttpRequest,
    todo_service: web::Data<TodoService>,
    todo_request: web::Json<CreateTodoRequest>,
) -> AppResult<HttpResponse> {
    let extensions = req.extensions();
    let auth_token = extensions.get::<AuthToken>().unwrap();
    
    info!("Preparing create transaction for user: {}", auth_token.public_key);
    
    let prepared_transaction = todo_service
        .prepare_create_transaction(&auth_token.public_key, todo_request.into_inner())
        .await?;
    
    Ok(HttpResponse::Ok().json(prepared_transaction))
}

#[utoipa::path(
    post,
    path = "/api/transactions/prepare/update/{id}",
    params(
        ("id" = u64, Path, description = "Todo ID")
    ),
    request_body = UpdateTodoRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Transaction prepared successfully", body = PreparedTransaction),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 429, description = "Rate limit exceeded"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/prepare/update/{id}")]
pub async fn prepare_update_transaction(
    req: HttpRequest,
    path: web::Path<u64>,
    todo_service: web::Data<TodoService>,
    update_request: web::Json<UpdateTodoRequest>,
) -> AppResult<HttpResponse> {
    let extensions = req.extensions();
    let auth_token = extensions.get::<AuthToken>().unwrap();
    let todo_id = path.into_inner();
    
    info!("Preparing update transaction for todo {} and user: {}", todo_id, auth_token.public_key);
    
    let prepared_transaction = todo_service
        .prepare_update_transaction(&auth_token.public_key, todo_id, update_request.into_inner())
        .await?;
    
    Ok(HttpResponse::Ok().json(prepared_transaction))
}

#[utoipa::path(
    post,
    path = "/api/transactions/prepare/delete",
    request_body = DeleteTodoTransactionRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Transaction prepared successfully", body = PreparedTransaction),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 429, description = "Rate limit exceeded"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/prepare/delete")]
pub async fn prepare_delete_transaction(
    req: HttpRequest,
    todo_service: web::Data<TodoService>,
    delete_request: web::Json<DeleteTodoTransactionRequest>,
) -> AppResult<HttpResponse> {
    let extensions = req.extensions();
    let auth_token = extensions.get::<AuthToken>().unwrap();
    let todo_id = delete_request.todo_id;
    
    info!("Preparing delete transaction for todo {} and user: {}", todo_id, auth_token.public_key);
    
    let prepared_transaction = todo_service
        .prepare_delete_transaction(&auth_token.public_key, todo_id)
        .await?;
    
    Ok(HttpResponse::Ok().json(prepared_transaction))
}

#[utoipa::path(
    post,
    path = "/api/transactions/submit",
    request_body = SignedTransaction,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Transaction submitted successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 429, description = "Rate limit exceeded"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/submit")]
pub async fn submit_transaction(
    req: HttpRequest,
    todo_service: web::Data<TodoService>,
    signed_transaction: web::Json<SignedTransaction>,
) -> AppResult<HttpResponse> {
    let extensions = req.extensions();
    let auth_token = extensions.get::<AuthToken>().unwrap();
    
    info!("Submitting signed transaction for user: {}", auth_token.public_key);
    
    let signature = todo_service
        .submit_transaction(signed_transaction.into_inner())
        .await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({ "signature": signature })))
} 