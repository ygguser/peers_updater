use crate::peer::Peer;

pub fn add_remove_peers(peers: &Vec<Peer>, yggctl_path: &str, n_peers: u8) {
    // Removing old peers
    if let Ok(output) = std::process::Command::new(yggctl_path)
        .arg("getPeers")
        .output()
    {
        let str_ou = String::from_utf8_lossy(&output.stdout);
        let out_strings = str_ou.split("\n");
        let mut skip = true;
        for ou_s in out_strings {
            if skip {
                skip = false;
                continue;
            }
            let mut n_str: u8 = 0;
            let string_parts = ou_s.split("\t");
            for str_part in string_parts {
                n_str += 1;
                if n_str == 8 {
                    let _ = std::process::Command::new(yggctl_path)
                        .arg("removepeer")
                        .arg(format!("uri={}", str_part.trim()))
                        .output();
                    n_str = 0;
                }
            }
        }
    } else {
        eprintln!("Could not get peers using the \"getpeers\" method.");
        std::process::exit(1);
    }

    // Adding new peers
    let mut n_added: u8 = 0;
    for peer in peers {
        let _ = std::process::Command::new(yggctl_path)
            .arg("addpeer")
            .arg(format!("uri={}", peer.uri))
            .output();
        n_added += 1;
        if n_added == n_peers {
            break;
        }
    }
}
