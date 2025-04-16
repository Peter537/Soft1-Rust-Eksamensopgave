use druid::widget::{Button, Flex };
use druid::{Color, Widget, WidgetExt};

use crate::ui::AppState;
use crate::ui::Screen::{
    DriverListScreen, Leaderboard, Main, RaceScheduleScreen, TeamListScreen,
};

pub fn build_navbar() -> impl Widget<AppState> {
        
    let home_button = Button::new("🏠").on_click(|ctx, data: &mut AppState, _env| {
        data.current_screen = Main;
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

    let leaderboard_button = Button::new("Leaderboard").on_click(|ctx, data: &mut AppState, _env| {
        data.current_screen = Leaderboard;
        ctx.request_update();
    });

    let exit_button = Button::new("Exit Game").on_click(|_ctx, _data: &mut AppState, _env| {
        std::process::exit(0); // Clean exit
    });

    // Create left group (home + nav buttons)
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

    // Align the exit button to the far right with a spacer in between
    Flex::row()
        .with_flex_child(left_side, 1.0)
        .with_spacer(90.0)
        .with_child(exit_button)
        .padding(10.0)
        .background(Color::grey8(200))
}
