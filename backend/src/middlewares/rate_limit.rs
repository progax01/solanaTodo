use std::future::{ready, Ready};
use std::sync::Arc;
use std::num::NonZeroU32;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use governor::{
    clock::{DefaultClock, Clock},
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use log::warn;

use crate::{config::get_config, error::AppError};

pub struct RateLimit {
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl RateLimit {
    pub fn new() -> Self {
        let config = get_config();
        let rate = NonZeroU32::new(config.rate_limit.requests).unwrap_or(NonZeroU32::new(100).unwrap());
        let quota = Quota::per_minute(rate);
        let limiter = Arc::new(RateLimiter::direct(quota));
        
        Self { limiter }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimitMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddleware {
            service,
            limiter: self.limiter.clone(),
        }))
    }
}

pub struct RateLimitMiddleware<S> {
    service: S,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddleware<S>
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
        // Check if the request is rate limited
        match self.limiter.check() {
            Ok(_) => {
                // Not rate limited, continue with the request
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(negative) => {
                // Rate limited, return a 429 Too Many Requests error
                let clock = DefaultClock::default();
                let wait_time = negative.wait_time_from(clock.now());
                let wait_ms = wait_time.as_millis();
                
                warn!(
                    "Rate limit exceeded for IP: {}. Retry after {} ms",
                    req.connection_info().realip_remote_addr().unwrap_or("unknown"),
                    wait_ms
                );
                
                let err = AppError::RateLimitExceeded;
                Box::pin(async move { Err(err.into()) })
            }
        }
    }
} 