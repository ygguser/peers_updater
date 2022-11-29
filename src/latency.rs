use crate::peer::Peer;
use dns_lookup::lookup_host;
use std::net::{SocketAddr, TcpStream};
use std::time;

pub fn set_latency(peer: &mut Peer) {
    let ip_addr = match resolve(&peer.addr) {
        Some(_a) => _a,
        _ => {
            peer.is_alive = false;
            return;
        }
    };

    let addr = match format!("{}:{}", ip_addr, peer.port)
        .as_str()
        .parse::<SocketAddr>()
    {
        Ok(_a) => _a,
        _ => {
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
    peer.is_alive = true;
    peer.latency = now.elapsed().as_millis();
    drop(stream);
}

fn resolve(name: &String) -> Option<String> {
    let addr = match name.starts_with('[') {
        true => {
            //return Some(name[1..name.len() - 1].to_string());
            return Some(name.to_string());
        }
        _ => name,
    };

    let ips = match lookup_host(addr.as_str()) {
        Ok(_ips) => _ips,
        _ => return None,
    };

    match ips.first() {
        Some(_ip) => Some(_ip.to_string()),
        _ => None,
    }
}
