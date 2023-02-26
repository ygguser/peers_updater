static mut TMPDIR_COUNT: u8 = 0;

pub fn create_tmp_dir(
    in_dir: Option<&std::path::Path>,
) -> Result<std::path::PathBuf, std::io::Error> {
    let tmp_dir_path: std::path::PathBuf = if let Some(in_dir_val) = in_dir {
        std::path::PathBuf::from(in_dir_val).join(format!(
            "{}{:x}-{:x}",
            "peers_updater_",
            std::process::id(),
            {
                unsafe {
                    TMPDIR_COUNT += 1;
                    TMPDIR_COUNT
                }
            },
        ))
    } else {
        std::env::temp_dir().join(format!(
            "{}{:x}-{:x}",
            "peers_updater_",
            std::process::id(),
            {
                unsafe {
                    TMPDIR_COUNT += 1;
                    TMPDIR_COUNT
                }
            },
        ))
    };

    if let Err(_) = std::fs::create_dir_all(&tmp_dir_path) {
        eprintln!(
            "Failed to create a temporary directory ({}).",
            &tmp_dir_path.display()
        );
        std::process::exit(1);
    };

    Ok(tmp_dir_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tmp_dir() {
        let path = create_tmp_dir(None);
        println!("{:?}", path);
        let path_pb: std::path::PathBuf = path.unwrap();
        let metadata = std::fs::metadata(&path_pb).unwrap();
        assert!(metadata.is_dir());
        std::fs::remove_dir_all(&path_pb).unwrap();
    }

    #[test]
    fn test_create_tmp_dir_in_dir() {
        let path = create_tmp_dir(Some(std::path::Path::new("/tmp/123/321")));
        println!("{:?}", path);
        let path_pb: std::path::PathBuf = path.unwrap();
        let metadata = std::fs::metadata(&path_pb).unwrap();
        assert!(metadata.is_dir());
        std::fs::remove_dir_all(&path_pb).unwrap();
    }
}
