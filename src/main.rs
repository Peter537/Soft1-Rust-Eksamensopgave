mod backend;
mod database;
mod ui;
mod util;

use druid::{AppLauncher, PlatformError, WindowDesc};

fn main() -> Result<(), PlatformError> {
    util::appdata::create_files_if_not_exist(); // Ensure necessary directories and files exist

    // Define the main window with the UI from ui::build_ui
    let main_window = WindowDesc::new(ui::build_ui()).title("Formula One Manager");

    // Launch the app with the default application state
    AppLauncher::with_window(main_window).launch(ui::AppState::default())?;

    Ok(())
}
