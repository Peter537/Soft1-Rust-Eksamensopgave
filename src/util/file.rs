use reqwest::blocking::get;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

pub fn download_file(url: &str, dest: &PathBuf) -> Result<(), Box<dyn Error + Send>> {
    let mut response = get(url).map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
    let mut file = File::create(dest).map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
    copy(&mut response, &mut file).map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
    Ok(())
}
