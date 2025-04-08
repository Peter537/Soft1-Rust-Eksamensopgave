use druid::widget::{Button, Flex, Label};
use druid::Widget;

use crate::backend::appdata; // Import the appdata module for file operations

use super::AppState;

pub fn build_screen() -> impl Widget<AppState> {
    let existing_careers = appdata::get_existing_careers();

    let mut existing_careers_str = String::new();
    for career in &existing_careers {
        existing_careers_str.push_str(&format!("Career_{}.db\n", career));
    }

    // Label displaying existing careers
    let existing_careers_label = Label::new(existing_careers_str.clone())
        .with_text_size(20.0)
        .with_text_color(druid::Color::rgb8(0, 0, 0)); // Set text color to black

    let create_new_career_button =
        Button::new("Create New Career").on_click(|_ctx, _data: &mut AppState, _env| {
            // Logic to create a new career
            appdata::create_new_career(); // Call the function to create a new career
        });

    // Vertical layout for the widgets
    Flex::column()
        .with_child(existing_careers_label)
        .with_child(create_new_career_button)
}
