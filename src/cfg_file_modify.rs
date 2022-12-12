use crate::peer::Peer;
use nu_json::{Map, Value};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process;

pub fn add_peers_to_conf(
    peers: &Vec<Peer>,
    conf_obj: &mut Map<String, nu_json::Value>,
    conf_path: &PathBuf,
    n_peers: u8,
    always_in_p: Option<&String>,
    ignored_peers: Option<&String>,
    restart: bool,
) {
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

    //Convert json obj to string
    let cfg_txt = match nu_json::to_string(&conf_obj) {
        Ok(_ct) => _ct,
        Err(e) => {
            eprintln!("Failed to convert json object to string ({}).", e);
            process::exit(1);
        }
    };

    //Write to file
    let mut f = match File::create(&conf_path) {
        Ok(_f) => _f,
        Err(e) => {
            eprintln!("Failed to make changes to the file ({}).", e);
            process::exit(1);
        }
    };

    let _ = match f.write_all(cfg_txt.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "The changes could not be written to the configuration file ({}).",
                e
            );
            process::exit(1);
        }
    };

    //Restart if required
    if restart {
        #[cfg(not(target_os = "windows"))]
        let _ = std::process::Command::new("systemctl")
            .arg("restart")
            .arg("yggdrasil")
            .spawn();

        #[cfg(target_os = "windows")]
        {
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
