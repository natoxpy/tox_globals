use serde::{Serialize, Deserialize};

use crate::ctp::responses::ConnectedNodes;

use super::responses::Hardware;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CtpResponse {
    ConnectedNodes(ConnectedNodes),
    Hardware(Hardware),
    Empty,
    Mishandle(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    pub content: CtpResponse,
    pub code: u16, // 200, 404, 500
}

impl ResponseFormat {
    pub fn new(content: CtpResponse, code: u16) -> Self {
        Self {
            content,
            code,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn to_pretty_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}