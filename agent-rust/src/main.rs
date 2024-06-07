mod app;
mod endpoint_manager;

use crate::app::create_app;
use crate::endpoint_manager::EndpointManager;
use actix_web::HttpServer;
use dotenv::dotenv;
use log::{error, info};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::signal::unix::{signal, SignalKind};

struct AppState {
    endpoint_manager: Arc<Mutex<EndpointManager>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init(); // Initialize the logger if using env_logger
    info!("Logger initialized");

    let endpoint_manager = Arc::new(Mutex::new(EndpointManager::new()));
    let app_state = AppState {
        endpoint_manager: endpoint_manager.clone(),
    };

    let port: u16 = env::var("PORT").unwrap_or_else(|_| "3001".to_string()).parse().expect("PORT must be a number");
    let server = HttpServer::new(move || {
        create_app()
            .app_data(web::Data::new(app_state.clone()))
    })
    .bind(("127.0.0.1", port))?
    .run();

    info!("Listening on port {}", port);

    let endpoint_manager_clone = endpoint_manager.clone();
    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(endpoint_manager_clone.lock().unwrap().initialize_agent_config());
    });

    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to create SIGTERM signal handler");
    tokio::select! {
        _ = server => {},
        _ = sigterm.recv() => {
            info!("SIGTERM received.. shutting down");
        }
    }

    Ok(())
}

fn setup_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        error!("Uncaught exception: {:?}", panic_info);
        std::process::exit(1);
    }));
}

fn setup_unhandled_rejection_handler() {
    // Handle unhandled rejections if needed, similar to Node.js
    tokio::spawn(async {
        // Example placeholder
    });
}

fn main() {
    setup_panic_handler();
    setup_unhandled_rejection_handler();
    if let Err(e) = main() {
        error!("Application error: {:?}", e);
    }
}