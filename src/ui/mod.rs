use crate::ui::Screen::{
    DriverListScreen, Leaderboard, MainGameScreen, RaceScheduleScreen, TeamListScreen,
};
use chrono::{NaiveDate, Utc};
use druid::widget::{Button, Flex, ViewSwitcher};
use druid::{
    Color, Command, Data, DelegateCtx, Env, Handled, Lens, Selector, Target, Widget, WidgetExt,
};

pub const SET_CURRENT_DATE: Selector<String> = Selector::new("app.set-current-date");
pub const RESET_GAME_STATE: Selector = Selector::new("app.reset-game-state");
pub const SET_SCREEN: Selector<Screen> = Selector::new("app.set-screen");
pub const SHOW_ERROR: Selector<String> = Selector::new("app.show-error");

mod choose_team_screen;
mod driver_list_screen;
mod driver_screen;
mod leaderboard_screen;
mod loading_screen;
mod main_game_screen;
mod main_screen;
mod race_schedule_screen;
mod race_screen;
mod team_list_screen;
mod team_screen;

pub mod component;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_screen: Screen,
    pub game_number: String,
    pub selected_team: Option<String>,
    pub current_date: String,
    pub last_race_update_time: String,
    pub show_modal: bool,
}

#[derive(Clone, PartialEq, Eq, Data)]
pub enum Screen {
    Loading,
    Main,
    ChooseTeam,
    MainGameScreen,
    RaceScreen { race_id: u16 },
    Leaderboard,
    TeamScreen { team_id: u16 },
    TeamListScreen,
    DriverScreen { driver_id: u16 },
    DriverListScreen,
    RaceScheduleScreen,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            current_screen: Screen::Loading,
            game_number: String::new(),
            selected_team: None,
            show_modal: false,
            current_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().to_string(),
            last_race_update_time: Utc::now().to_string(),
        }
    }
}

pub fn build_ui() -> impl druid::Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| (data.current_screen.clone(), data.game_number.clone()),
        |(screen, _game_number), _data, _env| -> Box<dyn druid::Widget<AppState>> {
            fn with_navbar(inner: impl Widget<AppState> + 'static) -> impl Widget<AppState> {
                Flex::column()
                    .with_child(build_navbar())
                    .with_spacer(10.0)
                    .with_flex_child(inner, 1.0)
            }

            match screen {
                Screen::Loading => Box::new(loading_screen::build_screen()),
                Screen::Main => Box::new(main_screen::build_screen()),
                Screen::TeamScreen { team_id } => {
                    Box::new(with_navbar(team_screen::build_screen(team_id)))
                }
                Screen::ChooseTeam => Box::new(choose_team_screen::build_screen()),
                Screen::MainGameScreen => Box::new(with_navbar(main_game_screen::build_screen())),
                Screen::RaceScreen { race_id } => {
                    Box::new(with_navbar(race_screen::build_screen(*race_id)))
                }
                Screen::Leaderboard => Box::new(with_navbar(leaderboard_screen::build_screen())),
                Screen::TeamListScreen => Box::new(with_navbar(team_list_screen::build_screen())),
                Screen::DriverScreen { driver_id } => {
                    Box::new(with_navbar(driver_screen::build_screen(driver_id)))
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

fn build_navbar() -> impl Widget<AppState> {
    let home_button = Button::new("Home").on_click(|ctx, data: &mut AppState, _env| {
        data.current_screen = MainGameScreen;
        ctx.request_update();
    });

    let drivers_button = Button::new("Drivers").on_click(|ctx, data: &mut AppState, _env| {
        data.current_screen = DriverListScreen;
        ctx.request_update();
    });

    let teams_button = Button::new("Teams").on_click(|ctx, data: &mut AppState, _env| {
        data.current_screen = TeamListScreen;
        ctx.request_update();
    });

    let races_button = Button::new("Races").on_click(|ctx, data: &mut AppState, _env| {
        data.current_screen = RaceScheduleScreen;
        ctx.request_update();
    });

    let leaderboard_button =
        Button::new("Leaderboard").on_click(|ctx, data: &mut AppState, _env| {
            data.current_screen = Leaderboard;
            ctx.request_update();
        });

    let exit_button = Button::new("Exit Game").on_click(|ctx, _data: &mut AppState, _env| {
        ctx.submit_command(RESET_GAME_STATE);
        ctx.request_update();
    });

    let left_side = Flex::row()
        .with_child(home_button)
        .with_spacer(10.0)
        .with_child(drivers_button)
        .with_spacer(10.0)
        .with_child(teams_button)
        .with_spacer(10.0)
        .with_child(races_button)
        .with_spacer(10.0)
        .with_child(leaderboard_button);

    Flex::row()
        .with_flex_child(left_side, 1.0)
        .with_flex_spacer(1.0)
        .with_child(exit_button)
        .padding(10.0)
        .background(Color::rgba8(0, 0, 125, 60))
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
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(new_date) = cmd.get(SET_CURRENT_DATE) {
            data.current_date = new_date.clone();
            Handled::Yes
        } else if cmd.is(RESET_GAME_STATE) {
            data.selected_team = None;
            data.game_number = "temp-trigger".to_string();
            data.current_screen = Screen::Main;
            data.game_number.clear();
            Handled::Yes
        } else if let Some(new_screen) = cmd.get(SET_SCREEN) {
            data.current_screen = new_screen.clone();
            Handled::Yes
        } else if let Some(_) = cmd.get(SHOW_ERROR) {
            data.current_screen = Screen::Main;
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
