use tempfile::Builder;

pub fn create_tmp_dir(
    in_dir: Option<&std::path::Path>,
) -> Result<std::path::PathBuf, std::io::Error> {
    let tmp_dir: tempfile::TempDir = if in_dir.is_none() {
        Builder::new().prefix("peers_updater_").tempdir()?
    } else {
        tempfile::Builder::new()
            .prefix("peers_updater_")
            .tempdir_in(::std::env::current_dir()?)?
    };

    Ok(tmp_dir.into_path())
}
