use crate::controllers::agent_endpoint_controller::{AgentEndpointController,AgentConfig};
//use actix_web::dev::Url;
use log::{debug, error, info};
use serde_yaml;
use std::sync::{Arc};
use tokio::sync::RwLock;
use ngrok::prelude::*;
use url::Url;

pub struct EndpointManager {
    endpoints: Arc<RwLock<Vec<AgentConfig>>>,
}

impl EndpointManager {
    pub fn new() -> Self {
        Self {
            endpoints: Arc::new(RwLock::new(Vec::new())),
        }
    }

    //pub async fn initialize_agent_config(&self, fetch_agent_config: impl Fn() -> task::JoinHandle<Result<Vec<AgentConfig>, Box<dyn std::error::Error>>>) {
    pub async fn initialize_agent_config(&self, configs: Vec<AgentConfig>) {
        let agent_endpoint_controller = web::Data::new(AgentEndpointController::new(self));

        let response = agent_endpoint_controller.fetch_agent_config().await;
        if let Ok(configs) = response {
            let mut endpoints = self.endpoints.write().await;
            *endpoints = configs.into_iter().map(|mut config| {
                config.status = "offline".to_string();
                config.listener = None;
                config
            }).collect();
            info!("Agent config initialized.");
        } else {
            error!("Failed to fetch agent config.");
        }
    }
    pub async fn change_endpoint_status(&self, id: &str) -> Result<Vec<AgentConfig>, Box<dyn std::error::Error>> {
        let mut endpoints = self.endpoints.write().await;
        let success = false;
        println!("Success: {}", success);

        if let Some(endpoint) = endpoints.iter_mut().find(|e| e.id == id) {
            if endpoint.status == "offline" {
                debug!("{:?}", endpoint);
                let endpoint_yaml = match serde_yaml::from_str::<serde_yaml::Value>(&endpoint.endpoint_yaml) {
                    Ok(yaml) => yaml,
                    Err(e) => {
                        error!("Failed to parse YAML for endpoint {}: {}", id, e);
                        return Ok(endpoints.clone());
                    }
                };

                debug!("Starting endpoint {} with options: {:?}", endpoint.name, endpoint_yaml);
                // info!("Ingress established for endpoint {} at: {}", endpoint.name, listener.url());
                endpoint.status = "online".to_string();
                success = true;

                let sess = ngrok::Session::builder()
                .authtoken_from_env()
                .connect()
                .await?;     

                match sess.http_endpoint().listen_and_forward(Url::parse("https://localhost:8001")?).await {
                    Ok(listener) => {
                        info!("Ingress established for endpoint {} at: {}", endpoint.name, listener.url());
                        endpoint.status = "online".to_string();
                        endpoint.listener = Some(listener);
                        success = true;
                    }
                    Err(e) => {
                        error!("Failed to start listener for endpoint {}: {}", id, e);

                    }
                }
               

                
              

            } else {
                debug!("Stopping endpoint {}", endpoint.name);
                match endpoint.listener.close().await {
                    Ok(_) => {
                        info!("Ingress closed for endpoint {}", endpoint.name);
                        endpoint.status = "offline".to_string();
                        success = true;
                    }
                    Err(e) => {
                        error!("Failed to close listener for endpoint {}: {}", id, e);
                    }
                }
            }
        }

        Ok(endpoints.clone())
    }

    pub async fn get_endpoints(&self) -> Vec<AgentConfig> {
        let endpoints = self.endpoints.read().await;
        endpoints.clone()
    }

    pub async fn add_endpoint(&self,mut endpoint: AgentConfig) {
        endpoint.listener = None;
        endpoint.status = "offline".to_string();
        let mut endpoints = self.endpoints.write().await;
        endpoints.push(endpoint);
    }

    pub async fn delete_endpoint(&self, id: &str) {
        let mut endpoints = self.endpoints.write().await;
        endpoints.retain(|e| e.id != id);
    }
}
