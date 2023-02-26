use clap::{arg, value_parser, Arg};
use std::path::PathBuf;

pub fn build_args() -> clap::ArgMatches {
    clap::Command::new("Yggdrasil peers updater")
    .version(env!("CARGO_PKG_VERSION"))
    .author("YggUser (https://matrix.to/#/@ygguser:matrix.org)")
    .about("The Yggdrasil peers updater automatically updates the peers in the Yggdrasil configuration file and/or calls addPeer/removePeer from the Yggdrasil Admin API.{n}Source code: https://github.com/ygguser/peers_updater")
    .arg(
        arg!(
            -p --print "Print the peers sorted by latency. When using this parameter, all other parameters will be ignored."
        )
        .required(false)
    )
    .arg(
        Arg::new("config")
       .short('c')
       .long("config")
       .default_value(crate::defaults::DEF_CFG_PATH)
       .value_name("FILE")
       .help("The path to the Yggdrasil configuration file")
       .required(false)
       .value_parser(value_parser!(PathBuf)))
    .arg(
        arg!(
            -u --update_cfg "Make changes to the Yggdrasil configuration file. If not specified, no changes will be made to the file."
        )
        .required(false)
    )
    .arg(
        arg!(
            -a --api "Add/remove peers during execution (requires enabling the admin API)"
        )
        .required(false)
    )
    .arg(
        arg!(
            -n --number <VALUE> "The number of peers to add (excluding extra ones)"
        )
        .required(false)
        .default_value("3")        
    )
    .arg(
        arg!(
            -e --extra <VALUE> "A space-separated string with the URIs of the peers that should always be in the configuration"
        )
        .required(false))
    .arg(
        arg!(
            -i --ignore <VALUE> "A space-separated string of characters. Peers whose URIs contain combinations of this characters will not be added to the configuration"
        )
        .required(false))
    .arg(
        arg!(
            -I --ignore_country <VALUE> "A space-separated string containing the names of countries that will not be added to the configuration"
        )
        .required(false))
    .arg(
        arg!(
            -r --restart "Restart the Yggdrasil (systemd or windows) service"
        )
        .required(false)
    )
    .arg(
        arg!(
            -S --self_update "Self-updating of this utility. An executable file will be downloaded from the releases on GitHub (if a newer version is published there) and the current one will be replaced with a new one."
        )
        .required(false)
    )
    .get_matches()
}
