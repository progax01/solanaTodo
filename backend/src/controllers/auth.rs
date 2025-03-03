use actix_web::{post, web, HttpResponse};
use log::info;
use utoipa::OpenApi;

use crate::{
    error::AppResult,
    models::auth::{AuthRequest, AuthResponse},
    services::auth::AuthService,
};

#[utoipa::path(
    post,
    path = "/api/auth",
    request_body = AuthRequest,
    responses(
        (status = 200, description = "Successfully authenticated", body = AuthResponse),
        (status = 401, description = "Authentication failed"),
        (status = 429, description = "Rate limit exceeded"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/auth")]
pub async fn authenticate(
    auth_service: web::Data<AuthService>,
    auth_request: web::Json<AuthRequest>,
) -> AppResult<HttpResponse> {
    info!("Authentication request received");
    
    let auth_response = auth_service.authenticate(auth_request.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(auth_response))
} 