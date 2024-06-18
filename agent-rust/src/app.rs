use actix_web::{ App};

use crate::utils::config::load_config;

pub fn create_app() -> App<impl actix_web::dev::ServiceFactory> {
    App::new()
        .configure(load_config)
}
