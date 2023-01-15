use std::{net::SocketAddr};



// SECTION: Address

#[derive(Debug, Clone)]
pub enum Address {
    Client(SocketAddr),
    AllClient,
    AnyClient,
    Api,
    Server,
    None,
}

impl Address {
    pub fn get_socket_addr(&self) -> Option<SocketAddr> {
        match self.clone() {
            Self::Client(client) => Some(client),
            _ => None
        }
    }
}


// SECTION: Target
#[derive(Debug, Clone)]
pub struct Target {
    pub from: Address,
    pub to: Address
}

impl Target {
    pub fn new(from: Address, to: Address) -> Self {
        Self {
            from, 
            to
        }
    }

    pub fn flip(&self) -> Self {
        let norm = self.clone();

        Self {
            from: norm.to,
            to: norm.from
        }
    }
} 