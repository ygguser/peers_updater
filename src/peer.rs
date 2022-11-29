//#[derive(Debug)]
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
        uri: String,
        addr: String,
        port: String,
        //proto: String,
        region: String,
        country: String,
        is_alive: bool,
        latency: u128,
    ) -> Self {
        Peer {
            uri,
            addr,
            port,
            //proto,
            region,
            country,
            is_alive,
            latency,
        }
    }
}
