// Defines the application state and orchestrates UI components by switching between screens.

use druid::{widget::{Flex, ViewSwitcher}, Data, Lens, Widget}; // Added ViewSwitcher import
use crate::ui::component::navbar::build_navbar; // Make sure to import

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
            // Helper function to wrap with navbar
            fn with_navbar(inner: impl Widget<AppState> + 'static) -> impl Widget<AppState> {
                Flex::column()
                    .with_child(build_navbar())
                    .with_spacer(10.0)
                    .with_flex_child(inner, 1.0)
            }

            match screen {
                Screen::Main => Box::new(main_screen::build_screen()),

                Screen::TeamScreen { team_name } => {
                    Box::new(team_screen::build_screen(&team_name))
                }

                Screen::ChooseTeam => Box::new(with_navbar(choose_team_screen::build_screen())),
                Screen::MainGameScreen => Box::new(with_navbar(main_game_screen::build_screen())),
                Screen::RaceScreen { race_id } => {
                    Box::new(with_navbar(race_screen::build_screen(*race_id)))
                }

                Screen::Leaderboard => {
                    Box::new(with_navbar(leaderboard_screen::build_screen()))
                }
                Screen::TeamListScreen => {
                    Box::new(with_navbar(team_list_screen::build_screen()))
                }
                Screen::DriverScreen { driver_name } => {
                    Box::new(with_navbar(driver_screen::build_screen(&driver_name)))
                }
                Screen::DriverListScreen => {
                    Box::new(with_navbar(driver_list_screen::build_screen()))
                }
                Screen::RaceScheduleScreen => {
                    Box::new(with_navbar(race_schedule_screen::build_screen()))
                }
            }
        },
    )
}
