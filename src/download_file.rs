use std::fs::File;
use std::path::Path;
//use std::io::Write;

type Res<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn download_archive(url: &str, tmp_dir: &Path, fname: &str) -> Res<()> {
    let response = tiny_http_client::get(url)?;

    if response.status != 200 {
        return Err(format!("HTTP request failed: {}", response.status).into());
    }

    let mut file = File::create(tmp_dir.join(fname))?;

    std::io::copy(&mut response.body.as_slice(), &mut file)?;

    Ok(())
}
