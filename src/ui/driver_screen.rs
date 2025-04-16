use druid::widget::{Button, Flex, Label, CrossAxisAlignment, MainAxisAlignment};
use druid::Widget;

use super::AppState;
use super::Screen::Main;

use crate::util::image_loader::get_driver;
use crate::ui::component::table::make_table;

use crate::database::driver::get_driver_by_firstname;

pub fn build_screen(driver_name: &String) -> impl Widget<AppState> {

    let driver = match get_driver_by_firstname(driver_name) {
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
    right_column.add_child(Label::new(format!("Driver Date of Birth: {}", driver.date_of_birth)));
    right_column.add_child(Label::new(format!("Driver Racing Number: {}", driver.racing_number)));


    let mut left_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    left_column.add_child(Label::new("Season Info"));
    left_column.add_child(Label::new("*TODO Info: e.g overall points, Total points, etc."));
    left_column.add_spacer(10.0);

    left_column.add_child(Label::new("Results"));
    let cols = vec!["Race".to_string(), "Positions".to_string(), "Points For Race".to_string()];
    let data = vec![
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec!["4".to_string(), "5".to_string(), "6".to_string()],
        vec!["7".to_string(), "8".to_string(), "9".to_string()],
    ];

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
