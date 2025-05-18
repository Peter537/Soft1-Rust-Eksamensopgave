#![windows_subsystem = "windows"]

use crate::ui::AppState;
use druid::{AppLauncher, PlatformError, WindowDesc};
use ui::MyAppDelegate;

mod backend;
mod database;
mod model;
mod ui;
mod util;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui::build_ui()).title("Formula One Manager");
    AppLauncher::with_window(main_window)
        .delegate(MyAppDelegate::new())
        .launch(AppState::default())
        .expect("Failed to launch application");

    Ok(())
}
