mod backend;
mod database;
mod model;
mod ui;
mod util;

use druid::{AppLauncher, PlatformError, WindowDesc};
use ui::MyAppDelegate;

fn main() -> Result<(), PlatformError> {
    util::appdata::create_files_if_not_exist();

    let main_window = WindowDesc::new(ui::build_ui()).title("Formula One Manager");

    let delegate = MyAppDelegate::new();

    AppLauncher::with_window(main_window)
        .delegate(delegate)
        .launch(ui::AppState::default())?;

    Ok(())
}
