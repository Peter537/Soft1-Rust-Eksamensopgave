use druid::widget::{Button, Flex, Label};
use druid::{Widget, WidgetExt};

use crate::backend::race;
use crate::util::appdata; // Import the appdata module for file operations

use crate::database::connection::set_game_number;

use super::AppState;
use super::Screen::{
    ChooseTeam, DriverListScreen, DriverScreen, Leaderboard, MainGameScreen, RaceScheduleScreen,
    TeamListScreen, TeamScreen,
};

pub fn build_screen() -> impl Widget<AppState> {
    // for testing purposes, delete this later:
    set_game_number(1);

    let existing_careers = appdata::get_existing_careers();

    let mut existing_careers_str = String::new();
    for career in &existing_careers {
        existing_careers_str.push_str(&format!("Career_{}.db\n", career));
    }

    // Label displaying existing careers
    let existing_careers_label = Label::new(existing_careers_str.clone())
        .with_text_size(20.0)
        .with_text_color(druid::Color::rgb8(0, 0, 0));

    let create_new_career_button =
        Button::new("Create New Career").on_click(|_ctx, _data: &mut AppState, _env| {
            // Logic to create a new career
            appdata::create_new_career(); // Call the function to create a new career
            // set_game_number(1); // Set the game number to 1 for testing purposes

            _data.current_screen = ChooseTeam;
            _ctx.request_update();
            println!("New career created!");
        });

    let create_load_saved_game_button =
        Button::new("Load Saved Game").on_click(|_ctx, _data: &mut AppState, _env| {
            // Logic to load a saved game
            // set_game_number(1); // Set the game number to 1 for testing purposes
            // appdata::load_saved_game(); // Call the function to load a saved game

            _data.current_screen = MainGameScreen;
            _ctx.request_update();
            println!("Saved Game loaded!");
        });

    // Vertical layout for the widgets
    Flex::column()
        .with_child(existing_careers_label)
        .with_child(create_new_career_button)
        .with_spacer(20.0) // Add some space between the buttons
        .with_child(create_load_saved_game_button)
        .with_spacer(120.0) // Add some space between the buttons
        .with_child(temp_buttons().align_left())
}

// Temporary buttons for navigation to different screens that should be in the navigation bar
fn temp_buttons() -> impl Widget<AppState> {
    let leaderboared_button =
        Button::new("leaderboared").on_click(|_ctx, _data: &mut AppState, _env| {
            println!("leaderboared_button clicked!");
            _data.current_screen = Leaderboard;
            _ctx.request_update();
        });

    let team_screen_button =
        Button::new("team_screen").on_click(|_ctx, _data: &mut AppState, _env| {
            println!("team_screen_button clicked!");
            _data.current_screen = TeamScreen;
            _ctx.request_update();
        });

    let team_list_screen_button =
        Button::new("team_list_screen").on_click(|_ctx, _data: &mut AppState, _env| {
            println!("team_list_screen_button clicked!");
            _data.current_screen = TeamListScreen;
            _ctx.request_update();
        });

    let driver_screen_button =
        Button::new("driver_screen").on_click(|_ctx, _data: &mut AppState, _env| {
            println!("driver_screen_button clicked!");
            _data.current_screen = DriverScreen;
            _ctx.request_update();
        });

    let driver_list_screen_button =
        Button::new("driver_list_screen").on_click(|_ctx, _data: &mut AppState, _env| {
            println!("driver_list_screen_button clicked!");
            _data.current_screen = DriverListScreen;
            _ctx.request_update();
        });

    let race_schedule_screen_button =
        Button::new("race_schedule_screen").on_click(|_ctx, _data: &mut AppState, _env| {
            println!("race_schedule_screen_button clicked!");
            _data.current_screen = RaceScheduleScreen;
            _ctx.request_update();
        });

    let race_start_button =
        Button::new("race_start_button").on_click(|_ctx, _data: &mut AppState, _env| {
            println!("race_start_button clicked!");
            race::start_race(1);
        });

    Flex::row()
        .with_child(leaderboared_button)
        .with_spacer(20.0)
        .with_child(team_screen_button)
        .with_spacer(20.0)
        .with_child(team_list_screen_button)
        .with_spacer(20.0)
        .with_child(driver_screen_button)
        .with_spacer(20.0)
        .with_child(driver_list_screen_button)
        .with_spacer(20.0)
        .with_child(race_schedule_screen_button)
        .with_spacer(20.0)
        .with_child(race_start_button)
        .with_spacer(20.0)
}
