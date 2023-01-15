use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedNodes {
    pub nodes: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hardware {
    pub cpu: String,
    pub ram: String,
    pub storage: String,
}