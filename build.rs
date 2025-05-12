use std::io::Result;
use winresource::WindowsResource;

fn main() -> Result<()> {
    if cfg!(target_os = "windows") {
        WindowsResource::new().set_icon("img/icon.ico").compile()?;
    }

    Ok(())
}
