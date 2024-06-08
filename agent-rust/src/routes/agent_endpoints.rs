use actix_web::{web, HttpResponse, Responder};
use crate::controllers::agent_endpoint_controller::{get_agent_status, update_endpoint_status, get_endpoint_status, add_endpoint, delete_endpoint};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(get_agent_status))
            .route("/updateStatus/{endpointId}", web::patch().to(update_endpoint_status))
            .route("/getEndPointStatus/{agentId}", web::get().to(get_endpoint_status))
            .route("/addEndpoint", web::post().to(add_endpoint))
            .route("/deleteEndpoint/{endpointId}", web::delete().to(delete_endpoint))
    );
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(serde_json::json!({
        "error": "Not Found"
    }))
}
