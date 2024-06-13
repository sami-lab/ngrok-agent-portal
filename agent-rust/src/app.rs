use actix_web::{App, web, HttpServer};

mod routes;
mod utils;

pub fn create_app() -> App<()> {
    App::new()
        .configure(routes::configure)
}
