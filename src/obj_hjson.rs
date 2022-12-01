use crate::config::get_hjson_obj;
use crate::peer::Peer;
use nu_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process;

pub fn add_peers_to_conf(
    peers: &Vec<Peer>,
    conf: &PathBuf,
    n_peers: u8,
    always_in_p: Option<&String>,
    ignored_peers: Option<&String>,
    restart: bool,
    is_unix: bool,
) {
    let mut conf_obj = match get_hjson_obj(conf) {
        Ok(_o) => _o,
        _ => {
            eprintln!("The configuration file cannot be parsed.");
            process::exit(1);
        }
    };

    // Extract the array of peers
    let peers_val = match conf_obj.get_mut("Peers") {
        Some(_a) => _a,
        _ => {
            eprintln!("Couldn't get peers from the configuration file.");
            process::exit(1);
        }
    };
    let mp_array = match peers_val.as_array_mut() {
        Some(_mv) => _mv,
        _ => {
            eprintln!("Couldn't get peers from the configuration file (0002).");
            process::exit(1);
        }
    };

    mp_array.clear();

    let mut n_added: u8 = 0;
    for peer in peers {
        if let Some(ignored_peers_p) = ignored_peers {
            if ignored_peers_p.contains(&peer.uri) {
                continue;
            }
        }
        mp_array.push(Value::String(format!(
            "{}#{}/{}",
            peer.uri, peer.region, peer.country
        )));
        n_added += 1;
        if n_added == n_peers {
            break;
        }
    }

    //Always in
    if let Some(always_in) = always_in_p {
        let ai = always_in.split(" ");
        for ai_s in ai {
            mp_array.push(Value::String(format!("{}", ai_s)));
        }
    }

    if let Ok(cfg_txt) = nu_json::to_string(&conf_obj) {
        //Write to file
        if let Ok(mut f) = File::create(&conf) {
            if let Ok(_res) = f.write_all(cfg_txt.as_bytes()) {
            } else {
                eprintln!("The changes could not be written to the configuration file.");
                process::exit(1);
            }
        } else {
            eprintln!("The changes could not be written to the configuration file.");
            process::exit(1);
        }

        //Restart if required
        if restart {
            if is_unix {
                let _ = std::process::Command::new("systemctl")
                    .arg("restart")
                    .arg("yggdrasil")
                    .spawn();
            } else {
                let _ = std::process::Command::new("net")
                    .arg("stop")
                    .arg("yggdrasil")
                    .output();
                let _ = std::process::Command::new("net")
                    .arg("start")
                    .arg("yggdrasil")
                    .spawn();
            }
        }
    }
}
