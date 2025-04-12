use druid::widget::{Button, Flex, Label};
use druid::Widget;

use super::AppState;
use super::Screen::Main;

use crate::util::image_loader;

use crate::database;

pub fn build_screen() -> impl Widget<AppState> {
    let driver = match database::driver::get_driver_by_id(16) {
        Some(driver) => driver,
        None => {
            println!("Driver not found!");
            return Flex::column()
                .with_child(Label::new("Driver not found!"))
                .with_spacer(20.0);
        }
    };
    Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("Driver Screen"))
        .with_child(
            Button::new("Back to Main").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_screen = Main;
                _ctx.request_update();
            }),
        )
        .with_child(Label::new(format!("Driver: {}", driver.first_name)))
        .with_child(Label::new(format!("Driver: {}", driver.last_name)))
        .with_child(Label::new(format!("Driver ID: {}", driver.id)))
        .with_child(Label::new(format!("Driver Rating: {}", driver.rating)))
        .with_child(Label::new(format!(
            "Driver Date of Birth: {}",
            driver.date_of_birth
        )))
        .with_child(Label::new(format!(
            "Driver Racing Number: {}",
            driver.racing_number
        )))
        .with_child(Label::new(format!(
            "Driver Image Path: {}",
            driver.image_path
        )))
        .with_child(Label::new(format!(
            "Driver Country ID: {}",
            driver.country_id
        )))
        .with_child(image_loader::get_driver(driver.image_path.as_str()))
        .with_spacer(20.0)
}
