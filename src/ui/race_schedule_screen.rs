use super::component::goto::{goto_driver, goto_race};
use super::AppState;
use crate::database::race::get_race_schedule_info;
use crate::ui::component::table::make_table;
use druid::widget::{Flex, Scroll, SizedBox};
use druid::Widget;

pub fn build_screen() -> impl Widget<AppState> {
    Flex::column().with_spacer(20.0).with_child(
        SizedBox::new(
            Scroll::new(make_table(
                vec![
                    "Date".to_string(),
                    "Country".to_string(),
                    "Name".to_string(),
                    "Status".to_string(),
                    "Winner".to_string(),
                    "2nd".to_string(),
                    "3rd".to_string(),
                ],
                get_race_schedule_info(),
                vec![
                    (2, goto_race()),
                    (4, goto_driver()),
                    (5, goto_driver()),
                    (6, goto_driver()),
                ],
            ))
            .vertical(),
        )
        .height(500.0),
    )
}
