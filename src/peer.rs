use std::net::{TcpStream, ToSocketAddrs};
use std::string::String;
use std::time;

#[derive(Debug)]
pub struct Peer {
    pub uri: String,
    pub addr: String,
    pub port: String,
    //proto: String,
    pub region: String,
    pub country: String,
    pub is_alive: bool,
    pub latency: u128,
}

impl Peer {
    pub fn new(
        uri: &str,
        addr: &str,
        port: &str,
        //proto: String,
        region: String,
        country: String,
        is_alive: bool,
        latency: u128,
    ) -> Self {
        Peer {
            uri: String::from(uri),
            addr: String::from(addr),
            port: String::from(port),
            //proto,
            region,
            country,
            is_alive,
            latency,
        }
    }

    pub fn set_latency(&mut self) {
        let mut addrs_iter = match format!("{}:{}", self.addr, self.port).to_socket_addrs() {
            Ok(_a) => _a,
            _ => {
                self.is_alive = false;
                return;
            }
        };
        let addr = match addrs_iter.next() {
            Some(__sa) => __sa,
            _ => {
                self.is_alive = false;
                return;
            }
        };

        let now = time::Instant::now();

        let stream = match TcpStream::connect_timeout(&addr, time::Duration::from_secs(10)) {
            Ok(_s) => _s,
            _ => {
                return;
            }
        };
        self.is_alive = true;
        self.latency = now.elapsed().as_millis();
        drop(stream);
    }
}
