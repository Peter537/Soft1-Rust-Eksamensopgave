use druid::widget::{Button, Flex, Label, Scroll, SizedBox};
use druid::Widget;

use super::AppState;
use super::Screen::Main;
use crate::database::race::get_race_schedule_info;
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

    let race_schedule_data = get_race_schedule_info().unwrap();

    let data = if !race_schedule_data.is_empty() {
        race_schedule_data
            .iter()
            .map(|race| {
                vec![
                    race.0.clone(),
                    race.1.clone(),
                    race.2.clone(),
                    race.3.clone(),
                    race.4.clone(),
                    race.5.clone(),
                    race.6.clone(),
                ]
            })
            .collect()
    } else {
        col.iter()
            .map(|_| vec!["".to_string(); col.len()])
            .collect()
    };

    let table = make_table(col, data, vec![]);

    Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("Race Schedule Screen"))
        .with_child(
            Button::new("Back to Main").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_screen = Main;
                _ctx.request_update();
            }),
        )
        .with_spacer(20.0)
        .with_child(table)
}
