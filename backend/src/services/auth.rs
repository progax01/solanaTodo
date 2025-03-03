use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{error, info};

use crate::{
    config::get_config,
    error::AppError,
    models::auth::{AuthRequest, AuthResponse, AuthToken, Claims},
    services::solana::SolanaService,
};

#[derive(Clone)]
pub struct AuthService {
    solana_service: SolanaService,
}

impl AuthService {
    pub fn new(solana_service: SolanaService) -> Self {
        Self { solana_service }
    }

    // Generate a JWT token for a user
    pub async fn authenticate(&self, auth_request: AuthRequest) -> Result<AuthResponse, AppError> {
        let config = get_config();
        let AuthRequest {
            public_key,
            signature,
            timestamp,
        } = auth_request;

        // 1. Verify that the timestamp is within a reasonable range (24 hours)
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AppError::internal(e.to_string()))?
            .as_secs() as i64;

        let time_diff = (current_time - timestamp).abs();
        if time_diff > 86400 {
            // 24 hours in seconds
            return Err(AppError::auth("Authentication request expired =============="));
        }

        // 2. Construct the message that was signed
        let message = format!("Sign in to Solana Todo App: {}", timestamp);

        // 3. Verify the signature
        let is_valid = self
            .solana_service
            .verify_signature(&public_key, &message, &signature)?;

        if !is_valid {
            return Err(AppError::auth("Invalid signature"));
        }

        // 4. Generate JWT token
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AppError::internal(e.to_string()))?
            .as_secs();

        let expiration = current_time + config.jwt.expiration;
        let claims = Claims {
            sub: public_key.clone(),
            exp: expiration,
            iat: current_time,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt.secret.as_bytes()),
        )
        .map_err(|e| AppError::internal(format!("Failed to create JWT token: {}", e)))?;

        Ok(AuthResponse {
            token,
            expires_in: config.jwt.expiration,
            public_key,
        })
    }

    // Verify a JWT token and extract the AuthToken
    pub fn verify_token(&self, token: &str) -> Result<AuthToken, AppError> {
        let config = get_config();

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::auth(format!("Invalid token: {}", e)))?;

        Ok(AuthToken {
            public_key: token_data.claims.sub,
        })
    }
} 