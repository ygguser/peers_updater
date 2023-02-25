use crate::peer::Peer;
use std::net::{TcpStream, ToSocketAddrs};
use std::time;

pub fn set_latency(peer: &mut Peer) {
    let mut addrs_iter = match format!("{}:{}", peer.addr, peer.port).to_socket_addrs() {
        Ok(_a) => _a,
        _ => {
            peer.is_alive = false;
            return;
        }
    };
    let addr = match addrs_iter.next() {
        Some(__sa) => __sa,
        _ => {
            peer.is_alive = false;
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
