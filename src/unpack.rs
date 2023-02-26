use std::fs;
use std::path::Path;

pub fn unpack_archive(tmp_dir: &Path, fname: &str) -> std::io::Result<bool> {
    let file = fs::File::open(format!("{}/{}", tmp_dir.display(), fname))?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let full_path = format!("{}/{}", tmp_dir.display(), out_path.display());
        let out_path = std::path::Path::new(full_path.as_str());
        if (*file.name()).ends_with('/') {
            fs::create_dir_all(out_path)?;
        } else {
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(out_path)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(out_path, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(true)
}
