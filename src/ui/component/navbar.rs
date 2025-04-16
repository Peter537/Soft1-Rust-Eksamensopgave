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

    let exit_button = Button::new("Exit Game").on_click(|ctx, data: &mut AppState, _env| {
        data.current_screen = Main; // Return to the main screen
        ctx.request_update();
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
    .with_flex_child(left_side, 1.0) // Left side takes up all available space
    .with_flex_spacer(1.0)          // Spacer takes up remaining space
    .with_child(exit_button)        // Exit button is placed at the far right
    .padding(10.0)
    .background(Color::rgba8(0, 0, 125, 60)) // 80% opacity as 204 in 0-255 range
}
