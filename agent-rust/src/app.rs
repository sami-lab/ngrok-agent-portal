use actix_web::{web, App, HttpServer};

mod utils;

pub fn create_app() -> App {
    App::new()
        .configure(utils::config::load_config)
}
