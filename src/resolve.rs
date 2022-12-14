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
        Some(_ip) =>
            if      _ip.is_ipv6() { Some(format!("[{}]", _ip.to_string())) }
            else if _ip.is_ipv4() { Some(format!("{}"  , _ip.to_string())) }
            else                  { None                                   },
        _ => None,
    }
}
