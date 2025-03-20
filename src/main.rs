mod config;
mod errors;
mod routes;
mod models;
mod services;

use actix_web::{App, HttpServer, middleware::Logger};
use log::info;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file if it exists
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // Initialize the unit registry with available units
    models::init::init();
    
    // Load configuration
    let config = config::AppConfig::from_env();
    
    info!("Starting server at http://{}:{}", config.server.host, config.server.port);
    
    // Configure and start HTTP server
    HttpServer::new(move || {
        App::new()
            // Enable logger middleware
            .wrap(Logger::default())
            
            // Configure routes
            .configure(routes::configure_routes)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
