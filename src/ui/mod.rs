// Defines the application state and orchestrates UI components by switching between screens.

use druid::{widget::ViewSwitcher, Data, Lens}; // Added ViewSwitcher import

// Public submodules for screen-specific UI logic
pub mod main_screen;

// Application state struct, holding data shared across the UI
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_screen: Screen, // Tracks the active screen
    pub game_number: String,
}

// Enum to represent different screens in the app
#[derive(Clone, PartialEq, Eq, Data)]
pub enum Screen {
    Main,
    //Settings,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            current_screen: Screen::Main,
            game_number: String::new(),
        }
    }
}

// Constructs the root UI widget, switching between screens based on AppState::current_screen
pub fn build_ui() -> impl druid::Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.current_screen.clone(),
        |screen, _data, _env| -> Box<dyn druid::Widget<AppState>> {
            match screen {
                Screen::Main => Box::new(main_screen::build_screen()),
                //Screen::Settings => Box::new(settings_screen::build_settings_screen()),
            }
        },
    )
}
