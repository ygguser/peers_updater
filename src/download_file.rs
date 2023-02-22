use std::fs::File;
use std::path::PathBuf;

type Res<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn download_archive(tmp_dir: &PathBuf, fname: &str) -> Res<()> {
    let resp = attohttpc::get(
        "https://github.com/yggdrasil-network/public-peers/archive/refs/heads/master.zip",
    )
    .send()?;

    let file = File::create(format!("{}/{}", tmp_dir.display(), fname))?;
    resp.write_to(file)?;

    Ok(())
}
