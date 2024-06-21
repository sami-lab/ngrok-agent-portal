use crate::endpoints_manager::EndpointManager;
use actix_web::{ HttpResponse, Responder};
use log::{info,error}; // Import the `error` macro from the `log` crate
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use ngrok::Tunnel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentConfig {
    pub id: String,
    pub name: String,
    pub endpoint_yaml: String,
    pub status: String,
    #[serde(skip)] // This will skip serializing the listener field
    pub listener: Option<Tunnel<ngrok::Connection, ngrok::TunnelItem>>,
}

pub struct AgentEndpointController {
    pub endpoints_manager: Arc<Mutex<EndpointManager>>,
    pub client: Client,
}

impl AgentEndpointController {
    pub fn new(endpoints_manager: Arc<Mutex<EndpointManager>>) -> Self {
        Self {
            endpoints_manager,
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
                let mut endpoints_manager = self.endpoints_manager.lock().unwrap();
                endpoints_manager.initialize_agent_config(configs);
                info!("Agent config initialized.");
            }
            Err(e) => {
                error!("Failed to fetch agent config.");
            }
        }
    }

    pub async fn get_agent_status(&self) -> impl Responder {
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Connected"
        }))
    }

    pub async fn update_endpoint_status(&self, id: String) -> impl Responder {
        let endpoints_manager = self.endpoints_manager.lock().unwrap();
        match endpoints_manager.change_endpoint_status(&id).await {
            Ok(endpoints) => HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "data": { "doc": endpoints.iter().find(|e| e.id == id) }
            })),
            Err(_) => HttpResponse::NotFound().json(serde_json::json!({
                "success": false,
                "message": "Agent endpoint not updated"
            })),
        }
    }

    pub async fn get_endpoint_status(&self, agent_id: String, agent_token: String) -> impl Responder {
        if agent_id != std::env::var("AGENT_ID").unwrap() || agent_token != std::env::var("AGENT_TOKEN").unwrap() {
            return HttpResponse::NotFound().json(serde_json::json!({
                "success": false,
                "message": "Agent endpoint not found"
            }));
        }

        let endpoints_manager = self.endpoints_manager.lock().unwrap();
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": { "doc": endpoints_manager.get_endpoints() }
        }))
    }

    pub async fn add_endpoint(&self, endpoint: AgentConfig) -> impl Responder {
        let mut endpoints_manager = self.endpoints_manager.lock().unwrap();
        endpoints_manager.add_endpoint(endpoint);
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": { "doc": endpoints_manager.get_endpoints() }
        }))
    }

    pub async fn delete_endpoint(&self, id: String) -> impl Responder {
        let mut endpoints_manager = self.endpoints_manager.lock().unwrap();
        endpoints_manager.delete_endpoint(&id);
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": { "doc": endpoints_manager.get_endpoints() }
        }))
    }
}
