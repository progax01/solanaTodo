use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct AuthRequest {
    #[schema(example = "HXtBm8XZbxaTt41uqaKhwUAa6Z1aPyvJdsZVENiWsetg")]
    pub public_key: String,
    
    #[schema(example = "3AuheKDvzxG6QM4gQHPNFTQ5wGz3aEZnECJK5Lp1e5orTAsyUNZKrGZq25e3XPQZiLgVj7LNzjwERFvxdL4Zx54M")]
    pub signature: String,
    
    #[schema(example = "1625097600")]
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct AuthResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
    
    #[schema(example = "86400")]
    pub expires_in: u64,
    
    #[schema(example = "HXtBm8XZbxaTt41uqaKhwUAa6Z1aPyvJdsZVENiWsetg")]
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject (public key)
    pub exp: u64,    // expiration time
    pub iat: u64,    // issued at
}

// Token that will be attached to each request
#[derive(Debug, Clone)]
pub struct AuthToken {
    pub public_key: String,
} 