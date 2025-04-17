use druid::widget::{Button, Flex, Label, Scroll, SizedBox};
use druid::Widget;

use super::AppState;
use super::Screen::Main;
use crate::ui::component::table::make_table;

pub fn build_screen() -> impl Widget<AppState> {

    let col = vec![
        "Date".to_string(),
        "Country".to_string(),
        "Name".to_string(),
        "Status".to_string(),
        "Pos 1".to_string(),
        "Pos 2".to_string(),
        "Pos 3".to_string(),
    ];

    let data= col.iter().map(|_| {
        vec!["".to_string(); col.len()]
    }).collect::<Vec<Vec<String>>>();

    let table = make_table(col, data, vec![]);

    Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("Race/Schedule List Screen"))
        .with_child(
            Button::new("Back to Main").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_screen = Main;
                _ctx.request_update();
            }),
        )
        .with_spacer(20.0)
        .with_child(SizedBox::new(Scroll::new(table).vertical()).height(400.0))
}
