use std::fs;
use std::io;
use std::path::{Component, Path};

pub fn unpack_archive(tmp_dir: &Path, fname: &str, set_exec: bool) -> io::Result<bool> {
    let archive_path = tmp_dir.join(fname);
    let mut file = fs::File::open(archive_path)?;

    let archive = munzip::IterableArchive::new(&mut file)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    for entry in archive {
        let mut entry = entry
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        let filename = entry.filename();
        let path = Path::new(&filename);

        // Do not allow absolute paths or ".." components.
        if path.is_absolute()
            || path.components().any(|component| {
                matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_))
            })
        {
            continue;
        }

        let out_path = tmp_dir.join(path);

        if filename.ends_with('/') {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let buffer = entry
                .buffer()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

            fs::write(&out_path, buffer)?;

            //Set executable permissions for self-update
            #[cfg(unix)]
            if set_exec {
                use std::os::unix::fs::PermissionsExt;

                fs::set_permissions(
                    &out_path,
                    fs::Permissions::from_mode(0o755),
                )?;
            }
        }
    }

    Ok(true)
}
