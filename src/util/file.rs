use reqwest::blocking::get;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

pub fn download_file(url: &str, dest: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut response = get(url)?;
    let mut file = File::create(dest)?;
    copy(&mut response, &mut file)?;
    Ok(())
}
