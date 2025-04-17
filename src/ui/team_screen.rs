use druid::widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment};
use druid::Widget;

use crate::database::teams::get_team_info;
use crate::ui::component::table::make_table;
use crate::util::image_loader::get_team;

use super::AppState;
use super::Screen::Main;

pub fn build_screen(team_name: &String) -> impl Widget<AppState> {
    println!("Building team screen for {}", team_name);
    let team_name_display = team_name.replace("_", " ");

    let team_info = match get_team_info(&team_name_display) {
        Some(info) => info,
        None => {
            println!("Team not found!");
            return Flex::column()
                .with_child(Label::new("Team not found!"))
                .with_spacer(20.0);
        }
    };

    let mut left_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    left_column.add_child(Label::new("Season Info"));
    left_column.add_spacer(5.0);
    left_column.add_child(Label::new(
        "*TODO Info: e.g overall points, Total points, etc.",
    ));
    left_column.add_spacer(10.0);

    left_column.add_child(Label::new("Results"));
    let cols = vec![
        "Race".to_string(),
        "TeamPositions".to_string(),
        "Points For Race".to_string(),
    ];
    let data = vec![
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec!["4".to_string(), "5".to_string(), "6".to_string()],
        vec!["7".to_string(), "8".to_string(), "9".to_string()],
    ];

    let results_table = make_table(cols, data, vec![]);
    left_column.add_child(results_table);

    let mut right_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    right_column.add_child(get_team(team_name));
    right_column.add_spacer(5.0);
    right_column.add_child(Label::new("Team Info"));
    right_column.add_spacer(5.0);
    right_column.add_child(Label::new(format!("Team Name: {}", team_info.full_name)));
    right_column.add_child(Label::new(format!(
        "Team Short Name: {}",
        team_info.short_name
    )));
    right_column.add_child(Label::new(format!("Base: {}", team_info.base_city)));
    right_column.add_child(Label::new(format!("Power Unit: {}", team_info.power_unit)));
    right_column.add_child(Label::new(format!("Team Chief: {}", team_info.team_chief)));
    right_column.add_child(Label::new(format!(
        "First Entry: {}",
        team_info.first_entry
    )));
    right_column.add_child(Label::new(format!("Chassis: {}", team_info.chassis)));
    right_column.add_child(Label::new(format!("Team ID: {}", team_info.id)));
    //left_column.add_child(Label::new(format!("Team Image Path: {}", team_info.image_path)));
    right_column.add_child(Label::new(format!(
        "Team Country ID: {}",
        team_info.country_id
    )));

    let layout = Flex::row()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_flex_child(left_column, 1.0)
        .with_spacer(80.0)
        .with_flex_child(right_column, 1.0)
        .with_spacer(40.0);

    Flex::column()
        .with_child(Label::new(format!("Team: {}", team_name_display)).with_text_size(30.0))
        .with_spacer(20.0)
        .with_child(Label::new("Team Screen"))
        .with_child(
            Button::new("Back to Main").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_screen = Main;
                _ctx.request_update();
            }),
        )
        .with_spacer(20.0)
        .with_child(layout)
}
