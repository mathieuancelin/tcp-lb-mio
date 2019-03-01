use std::net::{SocketAddr, AddrParseError};
use std::str::FromStr;

pub trait GetBackend {
    fn get(&mut self) -> Option<SocketAddr>;
    fn add(&mut self, backend_str: &str) -> Result<(), AddrParseError>;
    fn remove(&mut self, backend_str: &str) -> Result<(), AddrParseError>;
}

pub struct RoundRobinBackend {
    backends: Vec<SocketAddr>,
    last_used: usize,
}

impl RoundRobinBackend {
    pub fn new(backends_str: Vec<String>) -> Result<RoundRobinBackend, AddrParseError> {
        let mut backends = Vec::new();
        for backend_str in backends_str {
            let backend_socket_addr: SocketAddr = try!(FromStr::from_str(&backend_str));
            backends.push(backend_socket_addr);
            info!("Load balancing server {:?}", backend_socket_addr);
        }
        Ok(RoundRobinBackend {
            backends: backends,
            last_used: 0,
        })
    }
}

impl GetBackend for RoundRobinBackend {
    fn get(&mut self) -> Option<SocketAddr> {
        if self.backends.is_empty() {
            return None;
        }
        self.last_used = (self.last_used + 1) % self.backends.len();
        self.backends.get(self.last_used).map(|b| b.clone())
    }

    fn add(&mut self, backend_str: &str) -> Result<(), AddrParseError> {
        let backend_socket_addr: SocketAddr = try!(FromStr::from_str(&backend_str));
        self.backends.push(backend_socket_addr);
        Ok(())
    }

    fn remove(&mut self, backend_str: &str) -> Result<(), AddrParseError> {
        let backend_socket_addr: SocketAddr = try!(FromStr::from_str(&backend_str));
        self.backends.retain(|&x| x != backend_socket_addr);
        Ok(())
    }
}
