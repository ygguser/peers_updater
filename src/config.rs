use nu_json::{Map, Value};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn read_config(path: &PathBuf) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(format!(r#"{}"#, buffer))
    //Ok(format!("{}", buffer))
}

pub fn get_hjson_obj(path: &PathBuf) -> io::Result<Map<String, Value>> {
    let txt = read_config(path)?;

    if let Ok(obj) = nu_json::from_str(&txt) {
        return Ok(obj);
    }

    eprintln!("Can't parse the config file!");
    std::process::exit(1);
}
