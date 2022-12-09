use dns_lookup::lookup_host;

pub fn resolve(name: &String) -> Option<String> {
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
