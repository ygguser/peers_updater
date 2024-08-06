use crate::peer::Peer;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use std::process;

struct PPFile {
    path: std::path::PathBuf,
    region: String,
    country: String,
}

impl PPFile {
    pub fn new(path: &std::path::PathBuf, region: &str, country: &str) -> Self {
        PPFile {
            path: std::path::PathBuf::from(path),
            region: String::from(region),
            country: String::from(country),
        }
    }
}

fn collect_files(
    dir: &std::path::PathBuf,
    file_patches: &mut Vec<PPFile>,
    ignored_countries: &Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = std::fs::metadata(&path)?;
        if metadata.is_file() {
            let country = match path.file_stem() {
                Some(_c) => match _c.to_str() {
                    Some(_co) => _co,
                    _ => "Unknown",
                },
                _ => "Unknown",
            };

            if ignored_countries.contains(&country) {
                continue;
            }

            let region = match dir.file_stem() {
                Some(_r) => match _r.to_str() {
                    Some(_reg) => _reg,
                    _ => "Unknown",
                },
                _ => "Unknown",
            };

            file_patches.push(PPFile::new(&path, region, country));
        } else if let Err(e) = collect_files(&path, file_patches, ignored_countries) {
            eprintln!("Failed to collect *.md files ({}).", e);
            process::exit(1);
        }
    }

    Ok(())
}

pub fn collect_peers(
    path: &PathBuf,
    v: &mut Vec<Peer>,
    ignored_peers_str: &str,
    ignored_countries_str: &str,
) -> io::Result<bool> {
    let re = match Regex::new(r"(tcp|tls|quic|ws|wss)://([a-z0-9\.\-:\[\]]+):([0-9]+)") {
        Ok(_r) => _r,
        Err(e) => {
            eprintln!("Failed to create an instance of the RegEx parser ({}).", e);
            process::exit(1);
        }
    };

    let ignored_peers: &Vec<&str> = &(ignored_peers_str.split(' ').collect());
    let ignored_countries: &Vec<&str> = &(ignored_countries_str.split(' ').collect());

    let mut pp_files: Vec<PPFile> = Vec::with_capacity(30);
    if let Err(e) = collect_files(path, &mut pp_files, ignored_countries) {
        eprintln!("Failed to collect *.md files ({}).", e);
        process::exit(1);
    }

    for pp_file in pp_files {
        // Reading a file
        if let Ok(lines) = read_lines(pp_file.path) {
            for line in lines.into_iter().flatten() {
                for peer_ in re.captures_iter(line.as_str()) {
                    let uri = match peer_.get(0) {
                        Some(_u) => _u.as_str(),
                        None => {
                            continue;
                        }
                    };
                    let mut skip = false;
                    for ig in ignored_peers.iter() {
                        if (!ig.is_empty()) && uri.contains(ig.replace('"', "").as_str()) {
                            skip = true;
                            break;
                        }
                    }
                    if skip {
                        continue;
                    }
                    v.push(Peer::new(
                        uri,
                        peer_.get(2).map_or("", |m| m.as_str()),
                        peer_.get(3).map_or("", |m| m.as_str()),
                        // peer_
                        //     .get(1)
                        //     .map_or("".to_string(), |m| m.as_str().to_string()),
                        pp_file.region.to_owned(),
                        pp_file.country.to_owned(),
                        false,
                        99999,
                    ));
                }
            }
        }
    }

    Ok(true)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
