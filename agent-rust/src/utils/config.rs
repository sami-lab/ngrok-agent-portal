use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use actix_files::Files;
use crate::routes::agent_endpoints;
use std::collections::HashMap;

pub fn load_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(agent_endpoints::config)
    );

    // Middleware
    cfg.app_data(web::JsonConfig::default().limit(4096))
        .wrap(middleware::Logger::default())
        .wrap(middleware::Compress::default())
        .wrap(Cors::permissive());

    // Static files
    cfg.service(Files::new("/public", "./public").show_files_listing());

    // Custom error handlers
    cfg.default_service(
        web::route().to(|| async {
            HttpResponse::NotFound().json(HashMap::from([("error", "Not Found")]))
        })
    );
}

pub fn create_app() -> App {
    App::new()
        .configure(load_config)
}
