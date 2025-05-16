use super::AppState;
use crate::database::teams::{get_team_base_by_team_id, get_team_info, get_team_season_info};
use crate::model::RaceInfo;
use crate::ui::component::goto::goto_race;
use crate::ui::component::table::make_table;
use crate::util::image_loader::{get_car, get_team};
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, Scroll, SizedBox};
use druid::Widget;

pub fn build_screen(team_id: &u16) -> impl Widget<AppState> {
    let team_info = get_team_info(&team_id).unwrap();
    let team_base = get_team_base_by_team_id(&team_id).unwrap();
    let season_info = get_team_season_info(&team_info.id, &2025).unwrap();

    let mut left_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    left_column.add_child(Label::new("Season Info:").with_text_size(20.0));
    left_column.add_spacer(5.0);
    left_column.add_child(Label::new(format!(
        "Overall Position: {}",
        season_info.overall_position
    )));
    left_column.add_child(Label::new(format!(
        "Total Points: {}",
        season_info.total_points
    )));
    left_column.add_spacer(10.0);

    left_column.add_child(Label::new("Results:").with_text_size(20.0));
    left_column.add_spacer(5.0);

    let team_results_data: Vec<Vec<String>> = season_info
        .races
        .iter()
        .map(|race_info: &RaceInfo| {
            vec![
                race_info.grand_prix_name.clone(),
                race_info.date.clone(),
                race_info
                    .team_positions
                    .iter()
                    .map(|&pos| pos.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                race_info.race_points.to_string(),
            ]
        })
        .collect();

    left_column.add_child(
        SizedBox::new(
            Scroll::new(make_table(
                vec![
                    "Race".to_string(),
                    "Date".to_string(),
                    "Positions".to_string(),
                    "Points".to_string(),
                ],
                team_results_data,
                vec![(0, goto_race())],
            ))
            .vertical(),
        )
        .height(400.0),
    );

    let mut right_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    right_column.add_child(get_team(&team_info.image_path_logo));
    right_column.add_child(get_car(&team_info.image_path_car));
    right_column.add_child(Label::new(format!("Full Name:\t{}", team_info.full_name)));
    right_column.add_child(Label::new(format!(
        "Base:\t\t{}, {}",
        team_base.city, team_base.country_name
    )));
    right_column.add_child(Label::new(format!("Team Chief:\t{}", team_info.team_chief)));
    right_column.add_child(Label::new(format!("Power Unit:\t{}", team_info.power_unit)));
    right_column.add_child(Label::new(format!("Chassis:\t{}", team_info.chassis)));
    right_column.add_child(Label::new(format!(
        "First Entry:\t{}",
        team_info.first_entry
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
        .with_child(Label::new(team_info.short_name).with_text_size(30.0))
        .with_spacer(20.0)
        .with_child(layout)
}
