use winresource::WindowsResource;

fn main() -> std::io::Result<()> {
    if cfg!(target_os = "windows") {
        WindowsResource::new().set_icon("img/icon.ico").compile()?;
    }
    Ok(())
}
