use clap::{arg, value_parser, Arg, ArgAction};
use std::path::PathBuf;

pub fn build_args(def_cfg_path: &'static str) -> clap::ArgMatches {
    clap::Command::new("Yggdrasil peers updater")
    .version("0.0.1")
    .author("YggUser (https://matrix.to/#/@ygguser:matrix.org)")
    .about("The Yggdrasil peers updater automatically updates the peers in the Yggdrasil configuration file and/or calls addPeer/removePeer from the Yggdrasil Admin API.{n}Source code: https://github.com/ygguser/peers_updater")
    .arg(
        arg!(
            -p --print "Print the peers sorted by latency. When using this parameter, all other parameters will be ignored."
        )
        .required(false)
        .action(ArgAction::SetTrue)
    )
    .arg(
        Arg::new("config")
       .short('c')
       .long("config")
       .default_value(def_cfg_path)
       .value_name("FILE")
       .help("The path to the Yggdrasil configuration file")
       .required(false)
       .value_parser(value_parser!(PathBuf)))
    .arg(
        arg!(
            -a --api "Add/remove peers during execution (requires enabling the admin API)"
        )
        .required(false)
    )
    .arg(
        arg!(
            -n --number <VALUE> "The number of peers to add"
        )
        .required(false)
        .default_value("3")        
    )
    .arg(
        arg!(
            -e --extra <VALUE> "A space-separated string with the URIs of the peers that should always be in the configuration."
        )
        .required(false))
    .arg(
        arg!(
            -r --restart "Restart the Yggdrasil (systemd or windows) service"
        )
        .required(false)
    )
    .get_matches()
}
