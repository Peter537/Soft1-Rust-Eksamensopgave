use super::AppState;
use crate::database::country::get_country_image_path;
use crate::database::driver::{get_driver_by_id, get_driver_contract, get_driver_season_info};
use crate::model::RaceInfo;
use crate::ui::component::goto::goto_race;
use crate::ui::component::table::make_table;
use crate::util::image_loader::{get_country, get_driver};
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, Scroll, SizedBox};
use druid::Widget;

pub fn build_screen(driver_id: &u16) -> impl Widget<AppState> {
    let driver = get_driver_by_id(driver_id).unwrap();
    let driver_contract = get_driver_contract(driver_id).unwrap();

    let mut right_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    right_column.add_child(get_driver(driver.image_path.as_str()));
    right_column.add_child(get_country(
        &get_country_image_path(&driver.country_id).unwrap(),
    ));
    right_column.add_spacer(10.0);
    right_column.add_child(Label::new(format!("Overall Rating:\t{}", driver.rating)));
    right_column.add_child(Label::new(format!(
        "Date of Birth:\t\t{}",
        driver.date_of_birth
    )));
    right_column.add_child(Label::new(format!(
        "Racing Number:\t{}",
        driver.racing_number
    )));
    right_column.add_spacer(10.0);
    right_column.add_child(Label::new(format!("Contract Details:")));
    right_column.add_spacer(5.0);
    right_column.add_child(Label::new(format!(
        "Start:\t\t\t{}",
        driver_contract.date_begin
    )));
    right_column.add_child(Label::new(format!(
        "End:\t\t\t{}",
        driver_contract.date_end
    )));
    right_column.add_child(Label::new(format!(
        "Monthly Wage:\t{}",
        driver_contract.monthly_wage
    )));

    let season_info = get_driver_season_info(&driver.id, &2025).unwrap();

    let mut left_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    left_column.add_child(Label::new("Season Info:").with_text_size(20.0));
    left_column.add_spacer(5.0);
    left_column.add_child(Label::new(format!(
        "Overall Position:\t{}",
        season_info.overall_position
    )));
    left_column.add_child(Label::new(format!(
        "Total Points:\t\t{}",
        season_info.total_points
    )));
    left_column.add_spacer(10.0);

    left_column.add_child(Label::new("Results:").with_text_size(20.0));
    left_column.add_spacer(7.5);

    let driver_results_data: Vec<Vec<String>> = season_info
        .races
        .iter()
        .map(|race_info: &RaceInfo| {
            vec![
                race_info.grand_prix_name.clone(),
                race_info.date.clone(),
                race_info.team_positions[0].to_string(),
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
                    "Position".to_string(),
                    "Points".to_string(),
                ],
                driver_results_data,
                vec![(0, goto_race())],
            ))
            .vertical(),
        )
        .height(400.0),
    );

    let layout = Flex::row()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_flex_child(left_column, 1.0)
        .with_spacer(80.0)
        .with_flex_child(right_column, 1.0)
        .with_spacer(40.0);

    Flex::column()
        .with_child(
            Label::new(format!("{} {}", driver.first_name, driver.last_name)).with_text_size(30.0),
        )
        .with_spacer(20.0)
        .with_child(layout)
}
