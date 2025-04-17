use druid::widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment};
use druid::Widget;

use crate::database::teams::get_team_info;
use crate::ui::component::table::make_table;
use crate::util::image_loader::get_team;
use crate::database::teams::get_team_season_info;
use crate::model::season::RaceInfo;

use super::AppState;
use super::Screen::Main;

pub fn build_screen(team_id: &i32) -> impl Widget<AppState> {
    println!("Building team screen for {}", team_id);

    let team_info = match get_team_info(&team_id) {
        Some(info) => info,
        None => {
            println!("Team not found!");
            return Flex::column()
                .with_child(Label::new("Team not found!"))
                .with_spacer(20.0);
        }
    };

    let season_info = get_team_season_info(team_info.id, 2025).unwrap();
    
    let mut left_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    left_column.add_child(Label::new("Season Info:").with_text_size(20.0));
    left_column.add_spacer(5.0);
    left_column.add_child(Label::new(format!("Overall Position: {}", season_info.overall_position )));
    left_column.add_child(Label::new(format!("Total Points: {}", season_info.total_points)));
    left_column.add_spacer(10.0);

    left_column.add_child(Label::new("Results:").with_text_size(20.0));
    left_column.add_spacer(5.0);
    let cols = vec![
        "Race".to_string(),
        "Date".to_string(),
        "TeamPositions".to_string(),
        "Points For Race".to_string(),
    ];
    
    let data: Vec<Vec<String>> = season_info.races.iter().map(|race_info: &RaceInfo| {
        vec![
            race_info.grand_prix_name.clone(),                    
            race_info.date.clone(),                               
            race_info.team_positions.iter()                       
                .map(|&pos| pos.to_string())
                .collect::<Vec<String>>()
                .join(","),                                       
            race_info.race_points.to_string(),                    
        ]
    }).collect();

    let results_table = make_table(cols, data, vec![]);
    left_column.add_child(results_table);

    let mut right_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    right_column.add_child(get_team(&team_info.image_path_logo));
    right_column.add_spacer(5.0);
    right_column.add_child(Label::new("Team Info"));
    right_column.add_spacer(5.0);
    right_column.add_child(Label::new(format!("Team Name: {}", team_info.full_name)));
    right_column.add_child(Label::new(format!("Team Short Name: {}", team_info.short_name)));
    right_column.add_child(Label::new(format!("Base: {}", team_info.base_city)));
    right_column.add_child(Label::new(format!("Power Unit: {}", team_info.power_unit)));
    right_column.add_child(Label::new(format!("Team Chief: {}", team_info.team_chief)));
    right_column.add_child(Label::new(format!("First Entry: {}", team_info.first_entry)));
    right_column.add_child(Label::new(format!("Chassis: {}", team_info.chassis)));
    right_column.add_child(Label::new(format!("Team ID: {}", team_info.id)));
    right_column.add_child(Label::new(format!("Team Country ID: {}",team_info.country_id)));

    let layout = Flex::row()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_flex_child(left_column, 1.0)
        .with_spacer(80.0)
        .with_flex_child(right_column, 1.0)
        .with_spacer(40.0);

    Flex::column()
        .with_child(Label::new(format!("Team: {}", team_info.short_name)).with_text_size(30.0))
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
