use actix_web::{web, HttpResponse};
use actix_web::middleware::{Logger, Compress};
use actix_web::web::JsonConfig;
use actix_cors::Cors;
use std::collections::HashMap;

use crate::routes::agent_endpoints;

#[derive(Clone)]
pub struct AppState {
    pub logger: Logger,
    // Add other shared state here if needed
}

pub fn load_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/v1/test")
            .route(web::get().to(|| async {
                HttpResponse::Ok().json(HashMap::from([
                    ("status", "Test Backend Success")
                ]))
            }))
    );

    // Load agent endpoints routes
    agent_endpoints::configure(cfg);

    // Example of setting up middleware
    cfg.app_data(JsonConfig::default().limit(4096))
       .wrap(Logger::default())
       .wrap(Compress::default())
       .wrap(Cors::default());

    // Static files
    //cfg.service(Files::new("/public", "./public").show_files_listing());

    // Custom error handlers
    cfg.default_service(
        web::route().to(|| async {
            HttpResponse::NotFound().json(HashMap::from([
                ("error", "Not Found")
            ]))
        })
    );
}
