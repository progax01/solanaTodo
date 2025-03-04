use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod controllers;
mod error;
mod middlewares;
mod models;
mod services;
mod utils;

use crate::{
    config::get_config,
    controllers::{
        authenticate, create_todo, delete_todo, get_todos, update_todo,
        prepare_create_transaction, prepare_update_transaction, prepare_delete_transaction, submit_transaction
    },
    middlewares::{Authentication, RateLimit},
    services::{AuthService, SolanaService, TodoService},
    utils::ApiDoc,
};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();

    let config = get_config();
    info!("Starting server at {}:{}", config.server.host, config.server.port);

    // Initialize services
    let solana_service = SolanaService::new().expect("Failed to initialize Solana service");
    let todo_service = TodoService::new(solana_service.clone());
    let auth_service = AuthService::new(solana_service);

    // Create the server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        // Configure the OpenAPI documentation
        let openapi = ApiDoc::openapi();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(Data::new(auth_service.clone()))
            .app_data(Data::new(todo_service.clone()))
            // Add OpenAPI Swagger UI
            .service(SwaggerUi::new("/api/docs/{_:.*}").url("/api/docs/openapi.json", openapi.clone()))
            // API routes
            .service(
                web::scope("/api")
                    // Public routes
                    .service(authenticate)
                    // Protected routes
                    .service(
                        web::scope("/todos")
                            .wrap(RateLimit::new())
                            .wrap(Authentication::new(auth_service.clone()))
                            .service(get_todos)
                            .service(create_todo)
                            .service(update_todo)
                            .service(delete_todo)
                    )
                    // Transaction routes
                    .service(
                        web::scope("/transactions")
                            .wrap(RateLimit::new())
                            .wrap(Authentication::new(auth_service.clone()))
                            .service(prepare_create_transaction)
                            .service(prepare_update_transaction)
                            .service(prepare_delete_transaction)
                            .service(submit_transaction)
                    )
            )
    })
    .bind((config.server.host.clone(), config.server.port))?
    .run()
    .await
}
