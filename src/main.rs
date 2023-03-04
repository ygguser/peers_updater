use crate::peer::Peer;

#[cfg(feature = "using_api")]
use nu_json::Map;

use std::fs;
use std::path::PathBuf;
use std::process;

#[cfg(any(feature = "updating_cfg", feature = "using_api"))]
mod cfg_file_read_write;

#[cfg(any(
    feature = "updating_cfg",
    feature = "using_api",
    feature = "self_updating"
))]
mod defaults;

#[cfg(feature = "using_api")]
mod using_api;

#[cfg(feature = "self_updating")]
mod self_updating;

mod clap_args;
mod download_file;
mod parsing_peers;
mod peer;
mod tmpdir;
mod unpack;

fn main() {
    let matches = clap_args::build_args();

    #[cfg(feature = "self_updating")]
    if matches.get_flag("self_update") {
        self_updating::self_update();
        process::exit(0);
    }

    let print_only = matches.get_flag("print");

    let update_cfg = if cfg!(feature = "updating_cfg") {
        matches.get_flag("update_cfg")
    } else {
        false
    };

    let use_api = if cfg!(feature = "using_api") {
        matches.get_flag("api")
    } else {
        false
    };

    if !(print_only || update_cfg || use_api) {
        println!("At least the `-p` option is expected.");
        println!("For more information try '-h'.");
        println!("Nothing to do, exit.");
        process::exit(0);
    }

    #[cfg(any(feature = "updating_cfg", feature = "using_api"))]
    let conf_path = match matches.get_one::<PathBuf>("config") {
        Some(conf_path) => conf_path,
        _ => {
            eprintln!("Can't get the configuration file default path.");
            process::exit(1);
        }
    };

    #[cfg(feature = "updating_cfg")]
    if update_cfg {
        // Checking if the file exists
        if !conf_path.exists() {
            eprintln!("The Yggdrasil configuration file does not exist.");
            process::exit(1);
        }

        // Checking write access to the configuration file
        if let Err(e) = check_permissions(conf_path) {
            eprintln!(
                "There is no write access to the Yggdrasil configuration file ({}).",
                e
            );
            process::exit(1);
        }
    }

    // Creating a temporary directory
    let tmp_dir = match tmpdir::create_tmp_dir(None) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Failed to create a temporary directory ({}).", e);
            process::exit(1);
        }
    };

    // Download the archive with peers
    if let Err(e) = download_file::download_archive(
        "https://github.com/yggdrasil-network/public-peers/archive/refs/heads/master.zip",
        &tmp_dir,
        "peers.zip",
    ) {
        eprintln!("Failed to download archive with peers ({}).", e);
        process::exit(1);
    }

    // Unpacking the downloaded archive
    if let Err(e) = crate::unpack::unpack_archive(&tmp_dir, "peers.zip") {
        eprintln!("Failed to unpack archive ({}).", e);
        process::exit(1);
    }

    // Deleting unnecessary files
    let _ = fs::remove_file(std::path::Path::new(
        format!("{}/public-peers-master/README.md", &tmp_dir.display()).as_str(),
    ));
    let _ = fs::remove_file(std::path::Path::new(
        format!("{}/peers.zip", &tmp_dir.display()).as_str(),
    ));
    let _ = fs::remove_dir_all(std::path::Path::new(
        format!("{}/public-peers-master/other", &tmp_dir.display()).as_str(),
    ));

    let peers_dir: PathBuf =
        std::path::Path::new(format!("{}/public-peers-master/", &tmp_dir.display()).as_str())
            .to_path_buf();

    let ignored_peers: &str = match matches.get_one::<String>("ignore") {
        Some(_i_p) => _i_p.as_str(),
        None => "",
    };

    let ignored_countries: &str = match matches.get_one::<String>("ignore_country") {
        Some(_i_c) => _i_c.as_str(),
        None => "",
    };

    // Collecting peers in a vector
    let mut peers: Vec<Peer> = Vec::new();
    if let Err(e) = crate::parsing_peers::collect_peers(
        &peers_dir,
        &mut peers,
        ignored_peers,
        ignored_countries,
    ) {
        eprintln!("Couldn't get peer addresses from downloaded files ({}).", e);
        process::exit(1);
    };

    // Deleting unnecessary files
    let _ = fs::remove_dir_all(std::path::Path::new(tmp_dir.as_path()));

    // Calculating latency
    std::thread::scope(|scope| {
        for peer in &mut peers {
            scope.spawn(move || {
                peer.set_latency();
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
    } else if update_cfg || use_api {
        #[cfg(any(feature = "updating_cfg", feature = "using_api"))]
        if let Some(number) = matches.get_one::<String>("number") {
            let n_peers: u8 = match number.parse() {
                Ok(_n) => _n,
                Err(e) => {
                    eprintln!(
                        "The number of peers must be in the range from 0 to 255 ({}).",
                        e
                    );
                    process::exit(1);
                }
            };

            //Reading the configuration file
            let cfg_txt = match cfg_file_read_write::read_config(conf_path) {
                Ok(_ct) => _ct,
                Err(e) => {
                    eprintln!("The configuration file cannot be read ({}).", e);
                    process::exit(1);
                }
            };

            let exrta_peers: Option<&String> = matches.get_one("extra");

            // Adding peers to the configuration file
            #[cfg(feature = "updating_cfg")]
            if update_cfg {
                cfg_file_read_write::add_peers_to_conf_new(
                    &peers,
                    conf_path,
                    n_peers,
                    exrta_peers,
                    &cfg_txt,
                );
            }

            //Restart if required
            if matches.get_flag("restart") {
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

            // Adding peers during execution
            #[cfg(feature = "using_api")]
            if use_api {
                //Parsing the configuration file
                let mut conf_obj: Map<String, nu_json::Value> = match nu_json::from_str(&cfg_txt) {
                    Ok(co) => co,
                    Err(e) => {
                        eprintln!("Can't parse the config file ({})!", e);
                        process::exit(1);
                    }
                };

                using_api::update_peers(&peers, &mut conf_obj, n_peers, exrta_peers);
            }
        }
    }
}

#[cfg(feature = "updating_cfg")]
fn check_permissions(path: &PathBuf) -> std::io::Result<bool> {
    let md = fs::metadata(path)?;
    let permissions = md.permissions();
    Ok(permissions.readonly())
}
