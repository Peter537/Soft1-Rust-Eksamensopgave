use super::{component::goto::goto_team_fullname, AppState};
use crate::database::teams::get_team_data;
use crate::ui::component::goto::{goto_driver, goto_team};
use crate::ui::component::table::make_table;
use druid::widget::{Flex, Scroll, SizedBox};
use druid::Widget;

pub fn build_screen() -> impl Widget<AppState> {
    Flex::column().with_spacer(20.0).with_child(
        SizedBox::new(
            Scroll::new(make_table(
                vec![
                    "Short Name".to_string(),
                    "Full Team Name".to_string(),
                    "Points".to_string(),
                    "Driver 1".to_string(),
                    "Driver 2".to_string(),
                ],
                get_team_data(),
                vec![
                    (0, goto_team()),
                    (1, goto_team_fullname()),
                    (3, goto_driver()),
                    (4, goto_driver()),
                ],
            ))
            .vertical(),
        )
        .height(500.0),
    )
}
