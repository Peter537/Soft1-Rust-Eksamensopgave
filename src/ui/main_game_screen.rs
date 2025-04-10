
use druid::widget::{Button, Flex, Label};
use druid::Widget;

use super::AppState;
use super::Screen::RaceScreen;


pub fn build_screen() -> impl Widget<AppState> {

    let new_action_button = 
        Button::new("New Action").on_click(|_ctx, _data: &mut AppState, _env| {
        // Logic for new action
        _data.current_screen = RaceScreen;
        _ctx.request_update();
        println!("New action triggered!");
    });

    Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("Main Game Screen"))
        .with_spacer(20.0) 
        .with_child(new_action_button)
}
