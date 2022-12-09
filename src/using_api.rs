use crate::defaults;
use crate::peer::Peer;
use nu_json::Map;
use std::net::{SocketAddr, TcpStream};
use std::os::unix::net::UnixStream;
use std::time;

enum SockAddr {
    Tcp(SocketAddr),
    Unix(String),
    None,
}

enum Connection {
    Tcp(TcpStream),
    Unix(UnixStream),
    None,
}

pub fn update_peers(
    peers: &Vec<Peer>,
    conf_obj: &mut Map<String, nu_json::Value>,
    n_peers: u8,
    always_in_p: Option<&String>,
    ignored_peers: Option<&String>,
    is_unix: bool,
) {
    let socket_addr = get_socket_addr(is_unix, conf_obj);

    let mut response = String::new();

    // Get peers
    request("{\"request\": \"getpeers\"}", &socket_addr, &mut response);
    if response.is_empty() {
        eprintln!("Can't get connected peers.");
        return;
    }

    // Removing old peers
    remove_peers(&mut response, &socket_addr);

    // Adding new peers
    let mut n_added: u8 = 0;
    for peer in peers {
        if let Some(ignored_peers_p) = ignored_peers {
            if ignored_peers_p.contains(&peer.uri) {
                continue;
            }
        }
        response.clear();
        request(
            format!(
                "{{\"request\": \"addpeer\", \"arguments\": {{\"uri\": \"{}\"}}}}",
                peer.uri
            )
            .as_str(),
            &socket_addr,
            &mut response,
        );
        n_added += 1;
        if n_added == n_peers {
            break;
        }
    }

    //Always in
    if let Some(always_in) = always_in_p {
        let ai = always_in.split(" ");
        for ai_s in ai {
            response.clear();
            request(
                format!(
                    "{{\"request\": \"addpeer\", \"arguments\": {{\"uri\": \"{}\"}}}}",
                    ai_s
                )
                .as_str(),
                &socket_addr,
                &mut response,
            );
        }
    }
}

fn socket_io<T: std::io::Write + std::io::Read>(
    conn: &mut T,
    req: &str,
    resp: &mut String,
) -> std::io::Result<()> {
    conn.write_all(req.as_bytes())?;
    conn.read_to_string(resp)?;

    Ok(())
}

fn request(req: &str, socket_addr: &SockAddr, resp: &mut String) {
    let connection = get_connection(socket_addr);

    match connection {
        Connection::Tcp(conn) => {
            let mut mut_conn = conn;
            let _ = match socket_io(&mut mut_conn, req, resp) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Socket I/O error ({}).", e);
                }
            };
        }
        Connection::Unix(conn) => {
            let mut mut_conn = conn;
            let _ = match socket_io(&mut mut_conn, req, resp) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Socket I/O error ({}).", e);
                }
            };
        }
        Connection::None => {
            eprintln!("Unable to connect to the administrator socket.");
        }
    };
}

fn remove_peer(peer_uri: &String, socket_addr: &SockAddr, resp: &mut String) {
    request(
        format!(
            "{{\"request\": \"removepeer\", \"arguments\": {{\"uri\": \"{}\"}}}}",
            peer_uri
        )
        .as_str(),
        &socket_addr,
        resp,
    );
}

fn remove_peers(getpeers_resp: &mut String, socket_addr: &SockAddr) {
    //parse to obj
    let connected_peers: Map<String, nu_json::Value> = match nu_json::from_str(getpeers_resp) {
        Ok(cp) => cp,
        Err(e) => {
            eprintln!("Error converting a json string to an object ({}).", e);
            return;
        }
    };

    let resp = match connected_peers.get("response") {
        Some(_a) => _a,
        _ => {
            eprintln!("Couldn't get response from the getpeers result.");
            return;
        }
    };

    let peers_val = match resp.as_object() {
        Some(pv) => match pv.get("peers") {
            Some(_a) => _a,
            _ => {
                eprintln!("Couldn't get peers from the response obj.");
                return;
            }
        },
        _ => {
            eprintln!("Couldn't get peers from the response obj (0002).");
            return;
        }
    };

    let mp_array = match peers_val.as_array() {
        Some(_mv) => _mv,
        _ => {
            eprintln!("Couldn't get peers array from the the response obj.");
            return;
        }
    };

    for peer in mp_array {
        let peer_obj = match peer.as_object() {
            Some(_po) => _po,
            _ => {
                //eprintln!("Couldn't get peer obj.");
                continue;
            }
        };

        let peer_uri = match peer_obj.get("remote") {
            Some(_pu) => _pu.to_string().replace("\"", ""),
            _ => {
                //eprintln!("Couldn't get peer uri.");
                continue;
            }
        };

        getpeers_resp.clear();
        remove_peer(&peer_uri, socket_addr, getpeers_resp);
    }
}

fn get_connection(sock_addr: &SockAddr) -> Connection {
    match sock_addr {
        SockAddr::Tcp(_sa) => {
            let _ = match TcpStream::connect_timeout(&_sa, time::Duration::from_secs(10)) {
                Ok(_s) => {
                    return Connection::Tcp(_s);
                }
                Err(e) => {
                    eprintln!("Failed to connect via TCP stream ({}).", e);
                    return Connection::None;
                }
            };
        }
        SockAddr::Unix(_sa) => {
            let _ = match UnixStream::connect(_sa) {
                Ok(_s) => {
                    return Connection::Unix(_s);
                }
                Err(e) => {
                    eprintln!("Failed to connect via unix domain socket ({}).", e);
                    return Connection::None;
                }
            };
        }
        SockAddr::None => {
            return Connection::None;
        }
    };
}

fn get_socket_addr(is_unix: bool, conf_obj: &mut Map<String, nu_json::Value>) -> SockAddr {
    //Extract value from conf_obj
    let mut _t_sa: String;
    let mut string_addr = if let Some(_string_addr) = conf_obj.get("AdminListen") {
        _t_sa = format!("{}", _string_addr).replace("\"", "");

        _t_sa
    } else {
        String::from(defaults::get_def_socket_addr(is_unix))
    };

    if string_addr.contains("unix://") {
        //unix domain socket
        return SockAddr::Unix(
            string_addr
                .replace("\"", "")
                .replace("unix://", "")
                .to_string(),
        );
    } else {
        //tcp
        let uri = match uriparse::URI::try_from(string_addr.as_str()) {
            Ok(_u) => _u,
            Err(e) => {
                eprintln!("Unable to parse socket URI ({}).", e);
                return SockAddr::None;
            }
        };

        let host = match uri.host() {
            Some(_h) => _h,
            _ => {
                eprintln!("Unable to parse socket URI (failed to get host from URI).");
                return SockAddr::None;
            }
        };

        let ip_addr = match crate::resolve::resolve(&host.to_string()) {
            Some(_a) => _a,
            _ => {
                eprintln!("Failed to resolve host from socket URI.");
                return SockAddr::None;
            }
        };

        let port = match uri.port() {
            Some(_p) => _p,
            _ => {
                eprintln!("Unable to parse socket URI (failed to get port from URI).");
                return SockAddr::None;
            }
        };

        string_addr = format!("{}:{}", ip_addr, port);

        let _socket_addr = match string_addr.as_str().parse::<SocketAddr>() {
            Ok(_a) => {
                return SockAddr::Tcp(_a);
            }
            _ => {
                eprintln!("Unknown type of administrative socket address.");
                return SockAddr::None;
            }
        };
    }
}
