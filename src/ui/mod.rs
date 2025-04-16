use chrono::NaiveDate;
use druid::Selector;
use druid::{widget::ViewSwitcher, Data, Lens};
use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};

pub const SET_CURRENT_DATE: Selector<String> = Selector::new("app.set-current-date");

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

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_screen: Screen,
    pub game_number: String,
    pub selected_team: Option<String>,
    pub current_date: String,
}

#[derive(Clone, PartialEq, Eq, Data)]
pub enum Screen {
    Main,
    ChooseTeam,
    MainGameScreen,
    RaceScreen { race_id: i32 },
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
            current_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().to_string(),
        }
    }
}

pub fn build_ui() -> impl druid::Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.current_screen.clone(),
        |screen, _data, _env| -> Box<dyn druid::Widget<AppState>> {
            match screen {
                Screen::Main => Box::new(main_screen::build_screen()),
                Screen::ChooseTeam => Box::new(choose_team_screen::build_screen()),
                Screen::MainGameScreen => Box::new(main_game_screen::build_screen()),
                Screen::RaceScreen { race_id } => Box::new(race_screen::build_screen(*race_id)),
                Screen::Leaderboard => Box::new(leaderboard_screen::build_screen()),
                Screen::TeamScreen { team_name } => Box::new(team_screen::build_screen(&team_name)),
                Screen::TeamListScreen => Box::new(team_list_screen::build_screen()),
                Screen::DriverScreen { driver_name } => {
                    Box::new(driver_screen::build_screen(&driver_name))
                }
                Screen::DriverListScreen => Box::new(driver_list_screen::build_screen()),
                Screen::RaceScheduleScreen => Box::new(race_schedule_screen::build_screen()),
            }
        },
    )
}

pub struct MyAppDelegate;

impl MyAppDelegate {
    pub fn new() -> Self {
        MyAppDelegate
    }
}

impl druid::AppDelegate<AppState> for MyAppDelegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(new_date) = cmd.get(SET_CURRENT_DATE) {
            data.current_date = new_date.clone();
            // ctx.request_update();
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
