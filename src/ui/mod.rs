// Defines the application state and orchestrates UI components by switching between screens.

use druid::{widget::ViewSwitcher, Data, Lens}; // Added ViewSwitcher import

// Public submodules for screen-specific UI logic
pub mod choose_team_screen;
pub mod main_game_screen;
pub mod main_screen;
pub mod race_screen;

pub mod driver_list_screen;
pub mod driver_screen;
pub mod leaderboard_screen;
pub mod race_schedule_screen;
pub mod team_list_screen;
pub mod team_screen;

pub mod component;

// Application state struct, holding data shared across the UI
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_screen: Screen, // Tracks the active screen
    pub game_number: String,
    pub selected_team: Option<String>,
}

// Enum to represent different screens in the app
#[derive(Clone, PartialEq, Eq, Data)]
pub enum Screen {
    Main,
    //Settings,
    ChooseTeam,
    MainGameScreen,
    RaceScreen { race_id: i32 },

    // from nav:
    Leaderboard,
    TeamScreen { team_name: String },
    TeamListScreen,
    DriverScreen { driver_name: String },
    DriverListScreen,
    RaceScheduleScreen,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            current_screen: Screen::Main,
            game_number: String::new(),
            selected_team: None,
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
                Screen::ChooseTeam => Box::new(choose_team_screen::build_screen()),
                Screen::MainGameScreen => Box::new(main_game_screen::build_screen()),
                Screen::RaceScreen { race_id } => Box::new(race_screen::build_screen(*race_id)),

                // Nav:
                Screen::Leaderboard => Box::new(leaderboard_screen::build_screen()),
                Screen::TeamScreen { team_name } => Box::new(team_screen::build_screen(&team_name)),
                Screen::TeamListScreen => Box::new(team_list_screen::build_screen()),
                Screen::DriverScreen { driver_name } => Box::new(driver_screen::build_screen(&driver_name)),
                Screen::DriverListScreen => Box::new(driver_list_screen::build_screen()),
                Screen::RaceScheduleScreen => Box::new(race_schedule_screen::build_screen()),
            }
        },
    )
}
