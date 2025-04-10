
use druid::widget::{Button, Flex, Label};
use druid::Widget;

use super::AppState;


pub fn build_screen() -> impl Widget<AppState> {

    let home_button = 
        Button::new("Home").on_click(|_ctx, _data: &mut AppState, _env| {
        // Logic for home button
        _data.current_screen = super::Screen::Main; // Go back to main screen
        _ctx.request_update();
        println!("Home button clicked!");
    });

    Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("Race Screen"))
        .with_spacer(20.0) 
        .with_child(Label::new("Picture of Curcuit"))
        .with_spacer(20.0) 
        .with_child(Label::new("Circuit Info"))
        .with_spacer(20.0) 
        .with_child(Label::new("Simulation Results"))
        .with_spacer(20.0) 
        .with_child(Label::new("Delete this btn later for home page:"))
        .with_child(home_button)
}
