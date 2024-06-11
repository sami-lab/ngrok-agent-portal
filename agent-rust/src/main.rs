use actix_web::{App, HttpServer, middleware::Logger};
use std::sync::{Arc, Mutex};
use env_logger::Env;

mod app;
mod controllers;
mod endpoints_manager;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let endpoint_manager = Arc::new(Mutex::new(endpoint_manager::EndpointManager::new()));
    let agent_endpoint_controller = controllers::agent_endpoint_controller::AgentEndpointController::new(endpoint_manager.clone());

    // Initialize agent configuration
    agent_endpoint_controller.initialize_agent_config().await;

    let address = "127.0.0.1:8080";
    println!("Starting server at: {}", address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(agent_endpoint_controller.clone()))
            .configure(routes::agent_endpoints::configure)
    })
    .bind(address)?
    .run()
    .await
}
