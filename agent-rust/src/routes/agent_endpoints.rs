use actix_web::{web};
use std::sync::{Arc, Mutex};
use crate::controllers::agent_endpoint_controller::{AgentEndpointController};
use crate::endpoints_manager::EndpointManager;

pub fn configure(cfg: &mut web::ServiceConfig, endpoint_manager: Arc<Mutex<EndpointManager>>) {
    let agent_endpoint_controller = web::Data::new(AgentEndpointController::new(endpoint_manager.clone()));

    cfg.service(
        web::scope("/")
            .route("/", web::get().to(move || async { agent_endpoint_controller.get_agent_status().await }))
            .route("/updateStatus/{endpointId}", web::patch().to(move |path: web::Path<String>| {
                let endpoint_id = path.into_inner();
                async move { agent_endpoint_controller.update_endpoint_status(endpoint_id).await }
            }))
            .route("/getEndPointStatus/{agentId}", web::get().to(move |path: web::Path<String>| {
                let agent_id = path.into_inner();
                async move { agent_endpoint_controller.get_endpoint_status(agent_id).await }
            }))
            .route("/addEndpoint", web::post().to(move || async { agent_endpoint_controller.add_endpoint().await }))
            .route("/deleteEndpoint/{endpointId}", web::delete().to(move |path: web::Path<String>| {
                let endpoint_id = path.into_inner();
                async move { agent_endpoint_controller.delete_endpoint(endpoint_id).await }
            }))
    );
}
