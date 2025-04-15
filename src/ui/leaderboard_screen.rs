use druid::widget::{Button, Flex, CrossAxisAlignment, MainAxisAlignment, Label, Scroll};
use druid::Widget;

use super::component::table::make_table;
use super::AppState;
use super::Screen::Main;

use crate::database::teams::get_top_teams_standings;
use crate::database::driver::get_top_driver_standings;

use crate::ui::component::goto::{goto_driver, goto_team};

pub fn build_screen() -> impl Widget<AppState> {

    let all_drivers = get_top_driver_standings(None).unwrap_or(vec![]);
    let driver_cols = vec!["#".to_string(), "Driver Name".to_string(), "Points".to_string()];

    // make dommain for the table if empty
    let mut data: Vec<Vec<String>> = driver_cols.iter()
        .map(|_| vec!["".to_string(); driver_cols.len()])
        .collect();

    if !all_drivers.is_empty() {
        data = all_drivers
            .iter()
            .map(|(n, driver_name, points)| vec![n.to_string(), driver_name.clone(), points.to_string()])
            .collect();
    }

    let driver_table = make_table(driver_cols, data, vec![(1, goto_driver())]);

    let mut right_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    right_column.add_child(Label::new("Top 3 drivers standings").with_text_size(20.0));
    right_column.add_spacer(5.0);
    right_column.add_flex_child(Scroll::new(driver_table).vertical(), 1.0);
    

    let all_teams = get_top_teams_standings(None).unwrap_or(vec![]);
    let team_cols = vec!["#".to_string(), "Team Name".to_string(), "Points".to_string()];
    
    let mut data: Vec<Vec<String>> = team_cols.iter()
        .map(|_| vec!["".to_string(); team_cols.len()])
        .collect();

    if !all_teams.is_empty() {
        data = all_teams
            .iter()
            .map(|(n, team_name, points)| vec![n.to_string(), team_name.clone(), points.to_string()])
            .collect();
    }

    let team_table = make_table(team_cols, data, vec![(1, goto_team())]);

    let mut left_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    left_column.add_child(Label::new("Top 3 teams standings").with_text_size(20.0));
    left_column.add_spacer(5.0);
    left_column.add_flex_child(Scroll::new(team_table).vertical(), 1.0);


    let layout = Flex::row()
    .main_axis_alignment(MainAxisAlignment::Center)
    .cross_axis_alignment(CrossAxisAlignment::Start)
    .must_fill_main_axis(true)
    .with_flex_child(right_column, 1.0)
    .with_spacer(40.0)
    .with_flex_child(left_column,1.0);
    
    // Remove later
    let back_to_main = Button::new("Back to Main").on_click(|_ctx, data: &mut AppState, _env| {
        data.current_screen = Main;
        _ctx.request_update();
    });

    Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("Leaderboard Screen"))
        .with_child(back_to_main)
        .with_spacer(40.0)
        .with_child(layout)
}
