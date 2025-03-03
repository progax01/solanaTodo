use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use log::error;

use crate::{
    error::AppError,
    models::auth::AuthToken,
    services::auth::AuthService,
};

pub struct Authentication {
    auth_service: AuthService,
}

impl Authentication {
    pub fn new(auth_service: AuthService) -> Self {
        Self { auth_service }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service,
            auth_service: self.auth_service.clone(),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
    auth_service: AuthService,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract the token from the Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|auth_header| {
                if auth_header.starts_with("Bearer ") {
                    Some(auth_header[7..].to_string())
                } else {
                    None
                }
            });

        // If no token is found, return a 401 Unauthorized error
        let token = match token {
            Some(token) => token,
            None => {
                let err = AppError::auth("No authorization token provided");
                return Box::pin(async move { Err(err.into()) });
            }
        };

        // Verify the token
        let auth_token = match self.auth_service.verify_token(&token) {
            Ok(token) => token,
            Err(err) => {
                error!("Token verification failed: {}", err);
                return Box::pin(async move { Err(err.into()) });
            }
        };

        // Attach the AuthToken to the request extensions
        req.extensions_mut().insert(auth_token);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
} 