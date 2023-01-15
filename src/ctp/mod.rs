

use self::{ctp_request::CtpRequest, ctp_response::CtpResponse, target::Target};

pub mod ctp_request;
pub mod ctp_response;
pub mod target;
pub mod responses;

#[derive(Debug, Clone)]
pub enum CTP {
    Request(ctp_request::CtpRequest, Target),
    Response(ctp_response::CtpResponse),
}

impl CTP {
    pub fn new_request(req: CtpRequest, target: Target) -> Self {
        Self::Request(req, target)
    }

    pub fn new_response(res: CtpResponse) -> Self {
        Self::Response(res)
    }

    pub fn to_response(&self) -> Option<CtpResponse> {
        match self {
            CTP::Response(res) => Some(res.clone()),
            _ => None,
        }
    }
}