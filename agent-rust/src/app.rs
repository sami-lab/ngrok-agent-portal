use actix_web::{web, App};

use crate::utils;

pub fn create_app() -> App {
    App::new()
        .configure(utils::config::load_config)
}
