
use druid::widget::{Button, Flex, Label};
use druid::Widget;

use super::AppState;

use super::Screen::MainGameScreen;

pub fn build_screen() -> impl Widget<AppState> {

    let start_game_button =
        Button::new("Start game").on_click(|_ctx, _data: &mut AppState, _env| {
            _data.current_screen = MainGameScreen;
            _ctx.request_update();
        });
    
    
    Flex::column()
        .with_child(Label::new("Choose your team"))
        .with_spacer(20.0) 
        .with_child(Label::new("Select a team from the list below:"))
        .with_spacer(20.0) 
        .with_child(start_game_button)
}
