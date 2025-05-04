use super::AppState;
use crate::database::driver::get_driver_data;
use crate::ui::component::goto::{goto_driver, goto_team};
use crate::ui::component::table::make_table;
use druid::widget::{Flex, Scroll, SizedBox};
use druid::Widget;

pub fn build_screen() -> impl Widget<AppState> {
    Flex::column().with_spacer(20.0).with_child(
        SizedBox::new(
            Scroll::new(make_table(
                vec![
                    "Name".to_string(),
                    "Racing Number".to_string(),
                    "Rating".to_string(),
                    "Country".to_string(),
                    "Team".to_string(),
                ],
                get_driver_data(),
                vec![(0, goto_driver()), (4, goto_team())],
            ))
            .vertical(),
        )
        .height(500.0),
    )
}
