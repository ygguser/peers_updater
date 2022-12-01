use crate::peer::Peer;
use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::process;
use tempfile::Builder;

mod clap_args;
mod config;
mod latency;
mod obj_hjson;
mod parsing;
mod pathes;
mod peer;
mod unpack;
mod using_api;

fn main() {
    let is_unix: bool = cfg!(unix);

    let def_cfg_path: &str = pathes::get_def_cfg_path(is_unix);
    let yggctl_path: &str = pathes::get_yggctl_path(is_unix);

    let matches = clap_args::build_args(def_cfg_path);

    let print_only = matches.get_flag("print");

    let conf = match matches.get_one::<PathBuf>("config") {
        Some(_c) => _c,
        _ => {
            eprintln!("Can't get the configuration file default path.");
            process::exit(1);
        }
    };

    if !print_only {
        // Checking if the file exists
        if !conf.exists() {
            eprintln!("The Yggdrasil configuration file does not exist.");
            process::exit(1);
        }

        // Checking write access to the configuration file
        let _t = match check_permissions(&conf) {
            Ok(_ro) => _ro,
            _ => {
                eprintln!("There is no write access to the Yggdrasil configuration file.");
                process::exit(1);
            }
        };
    }

    // Creating a temporary directory
    let tmp_dir = match create_tmp_dir() {
        Ok(val) => val,
        _ => {
            eprintln!("Failed to create a temporary directory.");
            process::exit(1);
        }
    };

    // Download the archive with peers
    let _res = match download_archive(&tmp_dir) {
        Ok(val) => val,
        _ => {
            eprintln!("Failed to download archive with peers.");
            process::exit(1);
        }
    };

    // Unpacking the downloaded archive
    let _res = match crate::unpack::unpack_archive(&tmp_dir) {
        Ok(val) => val,
        _ => {
            eprintln!("Failed to unpack archive.");
            process::exit(1);
        }
    };

    // Deleting unnecessary files
    let _ret = fs::remove_file(std::path::Path::new(
        format!("{}/public-peers-master/README.md", &tmp_dir.display()).as_str(),
    ));
    let _ret = fs::remove_file(std::path::Path::new(
        format!("{}/peers.zip", &tmp_dir.display()).as_str(),
    ));
    let _ret = fs::remove_dir_all(std::path::Path::new(
        format!("{}/public-peers-master/other", &tmp_dir.display()).as_str(),
    ));

    let peers_dir: PathBuf =
        std::path::Path::new(format!("{}/public-peers-master/", &tmp_dir.display()).as_str())
            .to_path_buf();

    // Collecting peers in a vector
    let mut peers: Vec<Peer> = Vec::new();
    match crate::parsing::collect_peers(&peers_dir, &mut peers) {
        Ok(_r) => _r,
        _ => {
            eprintln!("Couldn't get peer addresses from downloaded files.");
            process::exit(1);
        }
    };

    // Deleting unnecessary files
    let _ret = fs::remove_dir_all(std::path::Path::new(tmp_dir.as_path()));

    // Calculating latency
    std::thread::scope(|scope| {
        for peer in &mut peers {
            scope.spawn(move || {
                crate::latency::set_latency(peer);
            });
        }
    });

    //Sorting the vector
    peers.sort_by(|a, b| a.latency.cmp(&b.latency));

    // Printing data
    if print_only {
        println!(
            "{0:<60}|{1:<15}|{2:<15}|{3:<10}",
            "URI", "Region", "Country", "Latency"
        );
        println!("{0:-<100}", "-");
        for peer in peers {
            if !peer.is_alive {
                break;
            }
            println!(
                "{0:<60}|{1:<15}|{2:<15}|{3:<10}",
                peer.uri, peer.region, peer.country, peer.latency
            );
        }
        process::exit(0);
    }

    if let Some(number) = matches.get_one::<String>("number") {
        let n_peers: u8 = match number.parse() {
            Ok(_n) => _n,
            _ => {
                eprintln!("The number of peers must be in the range from 0 to 255.");
                process::exit(1);
            }
        };

        let exrta_peers: Option<&String> = matches.get_one::<String>("extra");
        let ignored_peers: Option<&String> = matches.get_one::<String>("ignore");

        // Adding peers to the configuration file
        obj_hjson::add_peers_to_conf(
            &peers,
            conf,
            n_peers,
            exrta_peers,
            ignored_peers,
            matches.get_flag("restart"),
            is_unix,
        );

        // Adding peers during execution
        let use_api = matches.get_flag("api");
        if use_api {
            using_api::add_remove_peers(&peers, yggctl_path, n_peers, exrta_peers, ignored_peers);
        }
    }
}

fn check_permissions(path: &PathBuf) -> io::Result<bool> {
    let md = fs::metadata(path)?;
    let permissions = md.permissions();
    Ok(permissions.readonly())
}

fn create_tmp_dir() -> io::Result<PathBuf> {
    let tmp_dir = Builder::new().prefix("peers_updater_").tempdir()?;
    Ok(tmp_dir.into_path())
}

fn download_archive(tmp_dir: &PathBuf) -> io::Result<bool> {
    let mut resp = reqwest::blocking::get(
        "https://github.com/yggdrasil-network/public-peers/archive/refs/heads/master.zip",
    )
    .expect("request failed");
    let mut out = File::create(format!("{}/peers.zip", tmp_dir.display()))?;
    io::copy(&mut resp, &mut out)?;
    Ok(true)
}
