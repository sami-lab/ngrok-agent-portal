use actix_web::{web};

use crate::controllers::agent_endpoint_controller::{AgentEndpointController, get_agent_status, update_endpoint_status, get_endpoint_status, add_endpoint, delete_endpoint};

pub fn configure(cfg: &mut web::ServiceConfig) {
    let agent_endpoint_controller = web::Data::new(AgentEndpointController::new());

    cfg.service(
        web::scope("/")
            .route("/", web::get().to(get_agent_status))
            .route("/updateStatus/{endpointId}", web::patch().to(update_endpoint_status))
            .route("/getEndPointStatus/{agentId}", web::get().to(get_endpoint_status))
            .route("/addEndpoint", web::post().to(add_endpoint))
            .route("/deleteEndpoint/{endpointId}", web::delete().to(delete_endpoint))
    );
}
