use crate::utils::logger;
use crate::endpoint_manager::EndpointManager;
use actix_web::{web, HttpResponse, Responder};
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use actix_web::error::ErrorInternalServerError;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentConfig {
    id: String,
    name: String,
    endpoint_yaml: String,
    status: String,
}

pub struct AgentEndpointController {
    endpoint_manager: Arc<Mutex<EndpointManager>>,
    client: Client,
}

impl AgentEndpointController {
    pub fn new(endpoint_manager: Arc<Mutex<EndpointManager>>) -> Self {
        Self {
            endpoint_manager,
            client: Client::new(),
        }
    }

    pub async fn fetch_agent_config(&self) -> Result<Vec<AgentConfig>, Box<dyn std::error::Error>> {
        let backend_url = std::env::var("BACKEND_URL")?;
        let agent_id = std::env::var("AGENT_ID")?;
        let agent_token = std::env::var("AGENT_TOKEN")?;

        let url = format!("{}/api/v1/endpoint/{}", backend_url, agent_id);
        let response = self.client.get(&url)
            .header("token", agent_token)
            .send()
            .await?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await?;
            let configs: Vec<AgentConfig> = serde_json::from_value(data["data"]["doc"].clone())?;
            Ok(configs)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to fetch agent config")))
        }
    }

    pub async fn initialize_agent_config(&self) {
        match self.fetch_agent_config().await {
            Ok(configs) => {
                let mut endpoint_manager = self.endpoint_manager.lock().unwrap();
                endpoint_manager.initialize_configs(configs);
                info!("Agent config initialized.");
            }
            Err(e) => {
                logger::error(&format!("Failed to fetch agent config: {}", e));
            }
        }
    }

    pub async fn get_agent_status(&self) -> impl Responder {
        HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Connected"
        }))
    }

    pub async fn update_endpoint_status(&self, id: String) -> impl Responder {
        let mut endpoint_manager = self.endpoint_manager.lock().unwrap();
        match endpoint_manager.change_endpoint_status(&id).await {
            Ok(endpoints) => HttpResponse::Ok().json(json!({
                "success": true,
                "data": { "doc": endpoints.iter().find(|e| e.id == id) }
            })),
            Err(_) => HttpResponse::NotFound().json(json!({
                "success": false,
                "message": "Agent endpoint not updated"
            })),
        }
    }

    pub async fn get_endpoint_status(&self, agent_id: String, agent_token: String) -> impl Responder {
        if agent_id != std::env::var("AGENT_ID").unwrap() || agent_token != std::env::var("AGENT_TOKEN").unwrap() {
            return HttpResponse::NotFound().json(json!({
                "success": false,
                "message": "Agent endpoint not found"
            }));
        }

        let endpoint_manager = self.endpoint_manager.lock().unwrap();
        HttpResponse::Ok().json(json!({
            "success": true,
            "data": { "doc": endpoint_manager.get_endpoints() }
        }))
    }

    pub async fn add_endpoint(&self, endpoint: AgentConfig) -> impl Responder {
        let mut endpoint_manager = self.endpoint_manager.lock().unwrap();
        endpoint_manager.add_endpoint(endpoint);
        HttpResponse::Ok().json(json!({
            "success": true,
            "data": { "doc": endpoint_manager.get_endpoints() }
        }))
    }

    pub async fn delete_endpoint(&self, id: String) -> impl Responder {
        let mut endpoint_manager = self.endpoint_manager.lock().unwrap();
        endpoint_manager.delete_endpoint(&id);
        HttpResponse::Ok().json(json!({
            "success": true,
            "data": { "doc": endpoint_manager.get_endpoints() }
        }))
    }
}
