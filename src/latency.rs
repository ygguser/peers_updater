use crate::peer::Peer;
use std::net::{SocketAddr, TcpStream};
use std::time;

pub fn set_latency(peer: &mut Peer) {
    let ip_addr = match crate::resolve::resolve(&peer.addr) {
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
