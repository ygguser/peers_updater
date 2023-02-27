use std::fs::File;
use std::path::Path;

type Res<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn download_archive(url: &str, tmp_dir: &Path, fname: &str) -> Res<()> {
    let response = tinyget::get(url).send()?;
    let mut file = File::create(format!("{}/{}", tmp_dir.display(), fname))?;
    std::io::copy(&mut response.as_bytes(), &mut file)?;

    Ok(())
}
