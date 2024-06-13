use crate::controllers::agent_endpoint_controller::AgentConfig;
use log::{debug, error, info};
use serde_yaml;
use std::sync::{Arc};
use tokio::sync::RwLock;
use tokio::task;

pub struct EndpointManager {
    endpoints: Arc<RwLock<Vec<AgentConfig>>>,
}

impl EndpointManager {
    pub fn new() -> Self {
        Self {
            endpoints: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn initialize_agent_config(
        &self,
        fetch_agent_config: impl Fn() -> task::JoinHandle<Result<Vec<AgentConfig>, Box<dyn std::error::Error>>>
    ) {
        let response = fetch_agent_config().await.unwrap().await;
        if let Ok(configs) = response {
            let mut endpoints = self.endpoints.write().await;
            *endpoints = configs.into_iter().map(|mut config| {
                config.status = "offline".to_string();
                config
            }).collect();
            info!("Agent config initialized.");
        } else {
            error!("Failed to fetch agent config.");
        }
    }
    pub async fn change_endpoint_status(&self, id: &str) -> Result<Vec<AgentConfig>, Box<dyn std::error::Error>> {
        let mut endpoints = self.endpoints.write().await;
        let mut success = false;

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
                // match ngrok::start_tunnel(endpoint_yaml).await {
                //     Ok(listener) => {
                //         info!("Ingress established for endpoint {} at: {}", endpoint.name, listener.url());
                //         endpoint.status = "online".to_string();
                //         success = true;
                //     }
                //     Err(e) => {
                //         error!("Failed to start listener for endpoint {}: {}", id, e);
                //     }
                // }
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

    pub async fn add_endpoint(&self, endpoint: AgentConfig) {
        let mut endpoints = self.endpoints.write().await;
        endpoints.push(endpoint);
    }

    pub async fn delete_endpoint(&self, id: &str) {
        let mut endpoints = self.endpoints.write().await;
        endpoints.retain(|e| e.id != id);
    }
}
