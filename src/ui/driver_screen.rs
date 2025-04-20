use druid::widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment};
use druid::Widget;

use super::AppState;
use super::Screen::Main;

use crate::ui::component::table::make_table;
use crate::util::image_loader::get_driver;
use crate::database::driver::{get_driver_by_id, get_driver_season_info};
use crate::model::season::RaceInfo;

pub fn build_screen(driver_id: &i32) -> impl Widget<AppState> {
    let driver = match get_driver_by_id(driver_id) {
        Some(driver) => driver,
        None => {
            println!("Driver not found!");
            return Flex::column()
                .with_child(Label::new("Driver not found!"))
                .with_spacer(20.0);
        }
    };

    let mut right_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    right_column.add_child(get_driver(driver.image_path.as_str()));
    right_column.add_spacer(10.0);
    right_column.add_child(Label::new("Driver Info"));
    right_column.add_child(Label::new(format!("Driver: {}", driver.first_name)));
    right_column.add_child(Label::new(format!("Driver: {}", driver.last_name)));
    right_column.add_child(Label::new(format!("Driver ID: {}", driver.id)));
    right_column.add_child(Label::new(format!("Driver Rating: {}", driver.rating)));
    right_column.add_child(Label::new(format!(
        "Driver Date of Birth: {}",
        driver.date_of_birth
    )));
    right_column.add_child(Label::new(format!(
        "Driver Racing Number: {}",
        driver.racing_number
    )));

    let season_info = get_driver_season_info(driver.id, 2025).unwrap();

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
            race_info.team_positions[0].to_string(),                                 
            race_info.race_points.to_string(),                    
        ]
    }).collect();

    let results_table = make_table(cols, data, vec![]);
    left_column.add_child(results_table);

    let layout = Flex::row()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_flex_child(left_column, 1.0)
        .with_spacer(80.0)
        .with_flex_child(right_column, 1.0)
        .with_spacer(40.0);

    Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("Driver Screen"))
        .with_child(
            Button::new("Back to Main").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_screen = Main;
                _ctx.request_update();
            }),
        )
        .with_spacer(20.0)
        .with_child(layout)
}
