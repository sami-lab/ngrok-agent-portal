use crate::utils::logger;
use actix_web::web::Data;
use serde_yaml;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Endpoint {
    pub id: String,
    pub name: String,
    pub endpoint_yaml: String,
    pub status: String,
    pub listener: Option<ngrok::Session>,
}

pub struct EndpointManager {
    endpoints: Arc<Mutex<Vec<Endpoint>>>,
}

impl EndpointManager {
    pub fn new() -> Self {
        EndpointManager {
            endpoints: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn initialize_agent_config(&self) {
        let response = agent_endpoint_controller::fetch_agent_config().await;
        if response.success {
            let mut endpoints = self.endpoints.lock().unwrap();
            *endpoints = response.data.into_iter().map(|x| {
                Endpoint {
                    id: x.id,
                    name: x.name,
                    endpoint_yaml: x.endpoint_yaml,
                    status: "offline".to_string(),
                    listener: None,
                }
            }).collect();
        }
    }

    pub async fn change_endpoints_status(&self, id: &str) -> (bool, Vec<Endpoint>) {
        let mut endpoints = self.endpoints.lock().unwrap();
        if let Some(endpoint) = endpoints.iter_mut().find(|e| e.id == id) {
            if endpoint.status == "offline" {
                logger::debug(&format!("{:?}", endpoint));
                let endpoint_yaml: serde_yaml::Value = match serde_yaml::from_str(&endpoint.endpoint_yaml) {
                    Ok(yaml) => yaml,
                    Err(yaml_error) => {
                        logger::error(&format!("Failed to parse YAML for endpoint {}: {}", id, yaml_error));
                        return (false, endpoints.clone());
                    }
                };

                logger::debug(&format!("Starting endpoint {} with options: {:?}", endpoint.name, endpoint_yaml));
                match ngrok::Session::builder()
                    .authtoken_from_env()
                    .parse(endpoint_yaml)
                    .start()
                    .await
                {
                    Ok(listener) => {
                        println!("Ingress established for endpoint {} at: {}", endpoint.name, listener.http().url());
                        endpoint.listener = Some(listener);
                        endpoint.status = "online".to_string();
                        return (true, endpoints.clone());
                    }
                    Err(err) => {
                        println!("Listener setup error: {}", err);
                        return (false, endpoints.clone());
                    }
                }
            } else {
                logger::debug(&format!("Stopping endpoint {}", endpoint.name));
                if let Some(listener) = endpoint.listener.take() {
                    if listener.close().await.is_ok() {
                        println!("Ingress closed");
                        endpoint.status = "offline".to_string();
                        return (true, endpoints.clone());
                    }
                }
                return (false, endpoints.clone());
            }
        }
        (false, endpoints.clone())
    }

    pub fn get_endpoints(&self) -> Vec<Endpoint> {
        self.endpoints.lock().unwrap().clone()
    }

    pub fn add_endpoint(&self, endpoint: Endpoint) -> Vec<Endpoint> {
        let mut endpoints = self.endpoints.lock().unwrap();
        endpoints.push(Endpoint {
            status: "offline".to_string(),
            listener: None,
            ..endpoint
        });
        endpoints.clone()
    }

    pub fn delete_endpoint(&self, id: &str) -> Vec<Endpoint> {
        let mut endpoints = self.endpoints.lock().unwrap();
        endpoints.retain(|e| e.id != id);
        endpoints.clone()
    }
}
