use std::env;
use std::sync::OnceLock;
use dotenv::dotenv;
use log::info;

#[derive(Clone, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub solana: SolanaConfig,
    pub jwt: JwtConfig,
    pub rate_limit: RateLimitConfig,
}

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub program_id: String,
    pub commitment: String,
}

#[derive(Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: u64,
}

#[derive(Clone, Debug)]
pub struct RateLimitConfig {
    pub requests: u32,
    pub duration: u64, // in seconds
}

pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        dotenv().ok();
        info!("Loading configuration from environment variables");

        Config {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(8080),
            },
            solana: SolanaConfig {
                rpc_url: env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://localhost:8899".to_string()),
                program_id: env::var("SOLANA_PROGRAM_ID").expect("SOLANA_PROGRAM_ID must be set"),
                commitment: env::var("SOLANA_COMMITMENT").unwrap_or_else(|_| "confirmed".to_string()),
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
                expiration: env::var("JWT_EXPIRATION").unwrap_or_else(|_| "86400".to_string()).parse().unwrap_or(86400),
            },
            rate_limit: RateLimitConfig {
                requests: env::var("RATE_LIMIT_REQUESTS").unwrap_or_else(|_| "100".to_string()).parse().unwrap_or(100),
                duration: env::var("RATE_LIMIT_DURATION").unwrap_or_else(|_| "60".to_string()).parse().unwrap_or(60),
            },
        }
    })
} 