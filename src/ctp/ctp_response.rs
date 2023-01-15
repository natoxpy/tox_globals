use serde::{Serialize, Deserialize};

use crate::ctp::responses::ConnectedNodes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CtpResponse {
    ConnectedNodes(ConnectedNodes),
    Empty,
    Mishandle(String),
}

impl CtpResponse {
    pub fn convert_to_json(&self) -> String {
        match self {
            CtpResponse::ConnectedNodes(res) => serde_json::to_string(&res).unwrap(),
            CtpResponse::Empty => String::from(""),
            CtpResponse::Mishandle(err) => String::from(err),
        }
    }
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    pub body: CtpResponse,
    pub code: u16, // 200, 404, 500
}

impl ResponseFormat {
    pub fn new(body: CtpResponse, code: u16) -> Self {
        Self {
            body,
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