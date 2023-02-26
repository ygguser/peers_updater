// ---------------------------------------------------------------------------------------------
// Copyright (C) ygguser 2023.
// https://github.com/ygguser
//
// Distributed under the MIT License (license terms are at http://opensource.org/licenses/MIT).
// ---------------------------------------------------------------------------------------------

struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}
struct GitHubVersion {
    tag_name: String,
    url: String,
}

pub fn self_update() {
    //Current version
    let current_version_str = env!("CARGO_PKG_VERSION");
    let mut version_vec: Vec<&str> = current_version_str.split('.').collect();
    let mut major: u8 = version_vec.get(0).get_or_insert(&"0").parse().unwrap_or(0);
    let mut minor: u8 = version_vec.get(1).get_or_insert(&"0").parse().unwrap_or(0);
    let mut patch: u8 = version_vec.get(2).get_or_insert(&"0").parse().unwrap_or(0);
    let curr_version: Version = Version {
        major,
        minor,
        patch,
    };

    //Current target triple
    let target = env!("TARGET");

    //Path to the current executable file
    let exe_path = match std::env::current_exe() {
        Ok(_ep) => _ep,
        Err(e) => {
            eprintln!("Failed to get current exe path: {}", e);
            ::std::process::exit(1);
        }
    };

    //Latest version
    let latest_version_gh = match get_latest_version(target) {
        Ok(_lv) => _lv,
        Err(e) => {
            eprintln!("API request to api.github.com failed. ({}).", e);
            ::std::process::exit(1);
        }
    };

    version_vec = latest_version_gh.tag_name.split('.').collect();
    major = version_vec.get(0).get_or_insert(&"0").parse().unwrap_or(0);
    minor = version_vec.get(1).get_or_insert(&"0").parse().unwrap_or(0);
    patch = version_vec.get(2).get_or_insert(&"0").parse().unwrap_or(0);
    let latest_version: Version = Version {
        major,
        minor,
        patch,
    };

    //Compare versions
    if !(latest_version.major > curr_version.major
        || (latest_version.major == curr_version.major
            && latest_version.minor > curr_version.minor)
        || (latest_version.major == curr_version.major
            && latest_version.minor == curr_version.minor
            && latest_version.patch > curr_version.patch))
    {
        println!("The current version is the latest version.");
        ::std::process::exit(0);
    }

    println!(
        "New release found: {} --> {}",
        &current_version_str, &latest_version_gh.tag_name
    );

    println!("Downloading...");

    // Creating a temporary directory
    let tmp_dir = match crate::tmpdir::create_tmp_dir(exe_path.parent()) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Failed to create a temporary directory ({}).", e);
            ::std::process::exit(1);
        }
    };

    //Downloading
    let fname = format!("{}.zip", target);
    if let Err(e) = crate::download_file::download_archive(&latest_version_gh.url, &tmp_dir, &fname)
    {
        eprintln!("Failed to download archive ({}).", e);
        ::std::process::exit(1);
    }

    println!("Extracting...");

    //Extracting
    if let Err(e) = crate::unpack::unpack_archive(&tmp_dir, &fname) {
        eprintln!("Failed to unpack archive ({}).", e);
        ::std::process::exit(1);
    }

    // Deleting zip file
    let _ = std::fs::remove_file(std::path::Path::new(
        format!("{}/{}", &tmp_dir.display(), fname).as_str(),
    ));

    println!("Replacing binary file...");

    #[cfg(target_os = "windows")]
    let new_file = std::path::Path::new(
        format!("{}/{}", &tmp_dir.display(), crate::defaults::EXE_NAME).as_str(),
    )
    .to_path_buf();
    #[cfg(target_os = "windows")]
    let tmp_exec_file_name = std::path::Path::new(
        format!("{}/{}", &tmp_dir.display(), "peers_updater_old.exe").as_str(),
    )
    .to_path_buf();

    #[cfg(not(target_os = "windows"))]
    let tmp_exec_file_name =
        std::path::Path::new(format!("{}/{}", &tmp_dir.display(), "peers_updater_old").as_str())
            .to_path_buf();
    #[cfg(not(target_os = "windows"))]
    let new_file = std::path::Path::new(
        format!("{}/{}", &tmp_dir.display(), crate::defaults::EXE_NAME).as_str(),
    )
    .to_path_buf();

    replace_executable_file(&new_file, &exe_path, &tmp_exec_file_name);

    // Cleanup
    let _ = std::fs::remove_dir_all(&tmp_dir);

    println!("Done.");
}

fn get_latest_version(
    target: &str,
) -> std::result::Result<GitHubVersion, Box<dyn std::error::Error>> {
    let resp = attohttpc::get("https://api.github.com/repos/ygguser/peers_updater/releases/latest")
        .send()?;

    let obj: nu_json::Map<String, nu_json::Value> = match nu_json::from_str(&resp.text()?) {
        Ok(_o) => _o,
        Err(e) => {
            eprintln!("Error converting a json string to an object ({}).", e);
            ::std::process::exit(1);
        }
    };

    let tag_name = match obj.get("tag_name") {
        Some(_a) => _a.to_string().replace("\"", ""),
        _ => {
            eprintln!("Failed to get \"tag_name\" from API request result.");
            ::std::process::exit(1);
        }
    };

    let assets = match obj.get("assets") {
        Some(_a) => _a,
        _ => {
            eprintln!("Failed to get \"assets\" from API request result.");
            ::std::process::exit(1);
        }
    };

    let as_array = match assets.as_array() {
        Some(_as) => _as,
        _ => {
            eprintln!("Failed to get array of assets from API request result.");
            ::std::process::exit(1);
        }
    };

    let url = match get_asset_url(&as_array, target) {
        Some(_u) => _u,
        None => {
            eprintln!("No asset found for target: {}", target);
            ::std::process::exit(1);
        }
    };

    Ok(GitHubVersion { tag_name, url })
}

fn get_asset_url(as_array: &Vec<nu_json::Value>, target: &str) -> Option<String> {
    for asset in as_array {
        let as_obj = match asset.as_object() {
            Some(_ao) => _ao,
            _ => {
                continue;
            }
        };

        let as_url = match as_obj.get("browser_download_url") {
            Some(_pu) => _pu.to_string().replace("\"", ""),
            _ => {
                continue;
            }
        };

        if as_url.contains(target) {
            return Some(as_url);
        }
    }

    None
}

fn replace_executable_file(
    src: &std::path::PathBuf,
    dst: &std::path::PathBuf,
    tmp: &std::path::PathBuf,
) {
    if dst.exists() {
        if let Err(e) = std::fs::rename(dst, tmp) {
            eprintln!("Failed to move dst to tmp. ({}).", e);
            ::std::process::exit(1);
        }

        if let Err(e) = std::fs::rename(src, dst) {
            if let Err(e) = std::fs::rename(tmp, dst) {
                eprintln!("Failed to move from tmp to dst. ({}).", e);
            }
            eprintln!("Failed to replace the current executable file. ({}).", e);
            ::std::process::exit(1);
        }
    } else {
        let _ = std::fs::rename(src, dst);
    }
}
