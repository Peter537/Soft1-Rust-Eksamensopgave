use druid::widget::{Button, Flex, Label};
use druid::Widget;

use super::AppState;
use super::Screen::Main;


pub fn build_screen() -> impl Widget<AppState> {

    Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("Driver Screen"))
        .with_child(Button::new("Back to Main").on_click(|_ctx, data: &mut AppState, _env| {
            data.current_screen = Main;
            _ctx.request_update();
        }))
        .with_spacer(20.0) 
}