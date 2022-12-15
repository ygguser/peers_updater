use crate::peer::Peer;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use std::process;
use walkdir::WalkDir;

pub fn collect_peers(path: &PathBuf, v: &mut Vec<Peer>, ignored_peers: &str) -> io::Result<bool> {
    let re = match Regex::new(r"(tcp|tls)://([a-z0-9\.\-:\[\]]+):([0-9]+)") {
        Ok(_r) => _r,
        Err(e) => {
            eprintln!("Failed to parse files ({}).", e);
            process::exit(1);
        }
    };

    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            //println!("{}", file.path().display());
            let p: &std::path::Path = file.path();
            let country = match p.file_stem() {
                Some(_c) => match _c.to_os_string().into_string() {
                    Ok(_co) => _co,
                    _ => "Unknown".to_string(),
                },
                _ => "Unknown".to_string(),
            };
            let region = match p.parent() {
                Some(_r) => match _r.file_stem() {
                    Some(_re) => match _re.to_os_string().into_string() {
                        Ok(_reg) => _reg,
                        _ => "Unknown".to_string(),
                    },
                    _ => "Unknown".to_string(),
                },
                _ => "Unknown".to_string(),
            };

            // Reading a file
            if let Ok(lines) = read_lines(file.path()) {
                for line in lines {
                    if let Ok(str) = line {
                        for peer_ in re.captures_iter(str.as_str()) {
                            let ignored = ignored_peers.split(" ");
                            let uri = match peer_.get(0) {
                                Some(_u) => _u.as_str().to_string(),
                                None => {
                                    continue;
                                }
                            };
                            let mut skip = false;
                            for ig in ignored {
                                if uri.contains(ig) {
                                    skip = true;
                                    break;
                                }
                            }
                            if skip {
                                continue;
                            }
                            v.push(Peer::new(
                                uri,
                                peer_
                                    .get(2)
                                    .map_or("".to_string(), |m| m.as_str().to_string()),
                                peer_
                                    .get(3)
                                    .map_or("".to_string(), |m| m.as_str().to_string()),
                                // peer_
                                //     .get(1)
                                //     .map_or("".to_string(), |m| m.as_str().to_string()),
                                region.to_owned(),
                                country.to_owned(),
                                false,
                                99999,
                            ));
                        }
                    }
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
