// Defines the application state and orchestrates UI components by switching between screens.
use chrono::{NaiveDate, Utc};
use druid::{
    widget::{Button, Flex, ViewSwitcher, ZStack},
    Data, Lens, Selector, TimerToken, UnitPoint, Vec2, Widget, WidgetExt,
};
use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};
use std::time::Duration;

use crate::ui::component::navbar::build_navbar;
use crate::ui::component::{modal::build_modal, toast::build_toast};

pub const SET_CURRENT_DATE: Selector<String> = Selector::new("app.set-current-date");



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

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_screen: Screen,
    pub game_number: String,
    pub selected_team: Option<String>,
    pub current_date: String,
    pub last_race_update_time: String,
    pub toast_message: Option<String>,
    #[data(ignore)]
    pub toast_timer: Option<TimerToken>, // Timer for toast message

    pub show_modal: bool,
    pub show_toast: bool,
}



#[derive(Clone, PartialEq, Eq, Data)]
pub enum Screen {
    Main,
    ChooseTeam,
    MainGameScreen,
    RaceScreen { race_id: i32 },
    Leaderboard,
    TeamScreen { team_id: i32 },
    TeamListScreen,
    DriverScreen { driver_id: i32 },
    DriverListScreen,
    RaceScheduleScreen,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            current_screen: Screen::Main,
            game_number: String::new(),
            selected_team: None,
            toast_message: None,
            toast_timer: None,
            show_modal: false,
            show_toast: false,
            current_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().to_string(),
            last_race_update_time: Utc::now().to_string(),
        }
    }
}

pub fn build_ui() -> impl druid::Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.current_screen.clone(),
        |screen, _data, _env| -> Box<dyn druid::Widget<AppState>> {
            fn with_navbar(inner: impl Widget<AppState> + 'static) -> impl Widget<AppState> {
                Flex::column()
                    .with_child(build_navbar())
                    .with_spacer(10.0)
                    .with_flex_child(inner, 1.0)
            }

            let toast_button = Button::new("Show Toast")
                .on_click(|ctx, data: &mut AppState, _env| {
                    data.toast_message = Some("This is a toast notification!".into());
                    data.show_toast = true;
                    data.toast_timer = Some(ctx.request_timer(Duration::from_secs(3)));
                    ctx.request_update();
                })
                .padding(10.0)
                .center();

            let modal_button = Button::new("Show Modal")
                .on_click(|_ctx, data: &mut AppState, _env| {
                    data.show_modal = !data.show_modal;
                })
                .padding(10.0)
                .center();

            match screen {
                Screen::Main => {
                    let main_content = Flex::column()
                        .with_child(main_screen::build_screen())
                        .with_spacer(20.0)
                        .with_child(modal_button)
                        .with_child(toast_button)
                        .with_child(build_toast());

                    ZStack::new(main_content)
                        .with_child(build_modal(), Vec2::new(200.0, 200.0), Vec2::new(0.0, 0.0), UnitPoint::CENTER, Vec2::new(0.0, 0.0))
                        .boxed()
                }
                Screen::TeamScreen { team_id } => {
                    Box::new(with_navbar(team_screen::build_screen(team_id)))
                }
                Screen::ChooseTeam => {
                    let choose_team_content = choose_team_screen::build_screen();
                    Box::new(choose_team_content) // No navbar for the ChooseTeam screen
                }
                Screen::MainGameScreen => Box::new(with_navbar(main_game_screen::build_screen())),
                Screen::RaceScreen { race_id } => {
                    Box::new(with_navbar(race_screen::build_screen(*race_id)))
                }
                Screen::Leaderboard => Box::new(with_navbar(leaderboard_screen::build_screen())),
                Screen::TeamListScreen => Box::new(with_navbar(team_list_screen::build_screen())),
                Screen::DriverScreen { driver_id } => {
                    Box::new(with_navbar(driver_screen::build_screen(driver_id)))
                }
                Screen::DriverListScreen => Box::new(with_navbar(driver_list_screen::build_screen())),
                Screen::RaceScheduleScreen => {
                    Box::new(with_navbar(race_schedule_screen::build_screen()))
                }

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
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(new_date) = cmd.get(SET_CURRENT_DATE) {
            data.current_date = new_date.clone();
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
