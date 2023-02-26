use std::fs::File;
use std::path::Path;

type Res<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn download_archive(url: &str, tmp_dir: &Path, fname: &str) -> Res<()> {
    let resp = attohttpc::get(url).send()?;
    let file = File::create(format!("{}/{}", tmp_dir.display(), fname))?;
    resp.write_to(file)?;

    Ok(())
}
