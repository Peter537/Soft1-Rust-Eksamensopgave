use chrono::NaiveDate;
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment};
use druid::{Widget, WidgetExt};

use crate::ui::component::table::make_table;

use crate::database::config::{get_current_date, update_current_date};
use crate::database::driver::get_top_driver_standings;
use crate::database::race::get_next_race;
use crate::database::teams::{get_own_team_standing, get_top_teams_standings};
use crate::ui::component::goto::{goto_driver, goto_team};

use super::AppState;
use super::Screen::{MainGameScreen, RaceScreen};

pub fn build_screen() -> impl Widget<AppState> {
    let current_date = get_current_date().unwrap();
    let next_race = get_next_race().unwrap();
    let next_race_day = NaiveDate::parse_from_str(&next_race.date, "%Y-%m-%d")
        .ok()
        .unwrap();

    let current_date_clone = current_date.clone();
    let next_race_day_clone = next_race_day.clone();
    let next_race_id = next_race.id.clone();

    let new_action_button =
        Button::new("New Action").on_click(move |_ctx, _data: &mut AppState, _env| {
            // Logic for new action
            if &current_date_clone == &next_race_day_clone {
                _data.current_screen = RaceScreen {
                    race_id: next_race_id,
                };
                _ctx.request_update();
                println!("New action triggered!");
            } else {
                update_current_date(&next_race_day_clone);
                //_data.current_screen = MainGameScreen; // TODO: Find out how to update the screen
                _ctx.request_update();
                println!("New action triggered!");
            }
        });

    let cols = vec!["1".to_string(), "2".to_string()];
    let data = vec![vec!["x".to_string(), "x".to_string()]];

    let race_list = make_table(cols, data, vec![]);

    let mut column1 = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    column1.add_child(Label::new("Race List"));
    column1.add_spacer(5.0);
    column1.add_child(race_list);
    column1.add_spacer(10.0);

    // Column 2 - Top 3 drivers and teams standings
    let mut column2 = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    column2.add_child(Label::new("Top 3 drivers standings"));
    column2.add_spacer(5.0);

    let top_three_drivers = get_top_driver_standings(Some(3)).unwrap_or(vec![]);
    let cols = vec![
        "#".to_string(),
        "Driver Name".to_string(),
        "Points".to_string(),
    ];

    // make dommain for the table if empty
    let mut data: Vec<Vec<String>> = cols
        .iter()
        .map(|_| vec!["".to_string(); cols.len()])
        .collect();

    if !top_three_drivers.is_empty() {
        data = top_three_drivers
            .iter()
            .map(|(n, driver_name, points)| {
                vec![n.to_string(), driver_name.clone(), points.to_string()]
            })
            .collect();
    }

    let top_three_drivers = make_table(cols, data, vec![(1, goto_driver())]);

    column2.add_child(top_three_drivers);
    column2.add_spacer(10.0);

    let top_three_teams = get_top_teams_standings(Some(3)).unwrap_or(vec![]);
    let cols = vec![
        "#".to_string(),
        "Team Name".to_string(),
        "Points".to_string(),
    ];

    let mut data: Vec<Vec<String>> = cols
        .iter()
        .map(|_| vec!["".to_string(); cols.len()])
        .collect();

    if !top_three_teams.is_empty() {
        data = top_three_teams
            .iter()
            .map(|(n, team_name, points)| {
                vec![n.to_string(), team_name.clone(), points.to_string()]
            })
            .collect();
    }

    let top_three_teams = make_table(cols, data, vec![(1, goto_team())]);

    column2.add_child(Label::new("Top 3 team standings"));
    column2.add_spacer(5.0);
    column2.add_child(top_three_teams);
    //////////////////////////////

    // Column 3 - My team standings
    let (team_name, drivers, total_points) =
        get_own_team_standing().unwrap_or(("".to_string(), vec![], 0));

    let mut column3 = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    column3 = column3.with_child(Label::new("My Team").with_text_size(20.0));

    let col3_container = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new(format!("Team: {}", team_name)).with_text_size(16.0))
        .with_spacer(5.0)
        .with_child(Label::new(format!("Drivers: {:?}", drivers.join(", "))).with_text_size(16.0))
        .with_spacer(5.0)
        .with_child(Label::new(format!("Total Points: {}", total_points)).with_text_size(16.0))
        .border(druid::theme::BORDER_DARK, 1.0);

    column3.add_flex_child(col3_container, 1.0);

    let mut column4 = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    column4.add_child(Label::new(
        "Current Date: ".to_owned() + &current_date.to_string(),
    ));
    column4.add_child(Label::new(
        "Next Race Date: ".to_owned() + &next_race_day.to_string(),
    ));
    column4.add_child(new_action_button);

    let layout = Flex::row()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_flex_child(column1, 1.0)
        .with_spacer(40.0)
        .with_flex_child(column2, 1.0)
        .with_spacer(40.0)
        .with_flex_child(column3, 1.0)
        .with_spacer(40.0)
        .with_flex_child(column4, 1.0)
        .with_spacer(40.0);

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacer(20.0)
        .with_child(Label::new("Main Game Screen"))
        .with_spacer(20.0)
        .with_child(layout)
}
