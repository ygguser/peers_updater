use crate::peer::Peer;
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
                Some(c) => c.to_str().unwrap_or("Unknown"),
                None => "Unknown",
            };

            if ignored_countries.contains(&country) {
                continue;
            }

            let region = match dir.file_stem() {
                Some(r) => r.to_str().unwrap_or("Unknown"),
                None => "Unknown",
            };

            file_patches.push(PPFile::new(&path, region, country));
        } else if let Err(e) = collect_files(&path, file_patches, ignored_countries) {
            eprintln!("Failed to collect *.md files ({}).", e);
            process::exit(1);
        }
    }

    Ok(())
}

fn parse_peer_uri(uri: &str) -> Option<(&str, &str)> {
    // Removing the scheme
    let rest = uri
        .strip_prefix("tcp://")
        .or_else(|| uri.strip_prefix("tls://"))
        .or_else(|| uri.strip_prefix("quic://"))
        .or_else(|| uri.strip_prefix("ws://"))
        .or_else(|| uri.strip_prefix("wss://"))?;

    // Separating the authority from path/query/fragment
    let end = rest.find(&['/', '?', '#'][..]).unwrap_or(rest.len());
    let authority = &rest[..end];

    // IPv6: [2001:db8::1]:1234
    if authority.starts_with('[') {
        let close = authority.find(']')?;

        if authority.as_bytes().get(close + 1) != Some(&b':') {
            return None;
        }

        let host = &authority[..=close];
        let port = &authority[close + 2..];

        if port.is_empty() || !port.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }

        return Some((host, port));
    }

    // IPv4 or hostname
    let pos = authority.rfind(':')?;

    let host = &authority[..pos];
    let port = &authority[pos + 1..];

    if host.is_empty() || port.is_empty() {
        return None;
    }

    if !port.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }

    Some((host, port))
}

pub fn collect_peers(
    path: &PathBuf,
    v: &mut Vec<Peer>,
    ignored_peers_str: &str,
    ignored_countries_str: &str,
) -> io::Result<bool> {
    const SCHEMES: [&str; 5] = [
        "tcp://",
        "tls://",
        "quic://",
        "ws://",
        "wss://",
    ];

    let ignored_peers: Vec<&str> = ignored_peers_str.split(' ').collect();
    let ignored_countries: Vec<&str> = ignored_countries_str.split(' ').collect();

    let mut pp_files: Vec<PPFile> = Vec::with_capacity(30);

    if let Err(e) = collect_files(path, &mut pp_files, &ignored_countries) {
        eprintln!("Failed to collect *.md files ({}).", e);
        process::exit(1);
    }

    for pp_file in pp_files {
        if let Ok(lines) = read_lines(pp_file.path) {
            for line in lines.map_while(Result::ok) {
                let mut pos = 0;

                while pos < line.len() {
                    // Looking for the nearest URI scheme
                    let mut found: Option<usize> = None;

                    for scheme in SCHEMES {
                        if let Some(idx) = line[pos..].find(scheme) {
                            let abs = pos + idx;
                            found = match found {
                                Some(cur) if cur < abs => Some(cur),
                                _ => Some(abs),
                            };
                        }
                    }

                    let start = match found {
                        Some(v) => v,
                        None => break,
                    };

                    // URI ends at whitespace or end of line
                    let rest = &line[start..];
                    let end = rest.find(char::is_whitespace).unwrap_or(rest.len());

                    let uri = &rest[..end];

                    pos = start + end;

                    let (host, port) = match parse_peer_uri(uri) {
                        Some(v) => v,
                        None => continue,
                    };

                    let mut skip = false;

                    for ig in &ignored_peers {
                        let ig = ig.trim_matches('"');

                        if !ig.is_empty() && uri.contains(ig) {
                            skip = true;
                            break;
                        }
                    }

                    if skip {
                        continue;
                    }

                    v.push(Peer::new(
                        uri,
                        host,
                        port,
                        pp_file.region.clone(),
                        pp_file.country.clone(),
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
