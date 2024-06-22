use actix_web::{web, App};
use crate::utils::config::load_config;
use std::sync::{Arc, Mutex};
use crate::endpoints_manager::EndpointManager;

pub fn create_app(endpoint_manager: Arc<Mutex<EndpointManager>>) -> App<impl actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest>> {
    App::new()
        .app_data(web::Data::new(endpoint_manager))
        .configure(|cfg| load_config(cfg))
}
