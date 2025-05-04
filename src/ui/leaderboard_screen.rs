use super::component::table::make_table;
use super::AppState;
use crate::database::driver::get_top_driver_standings;
use crate::database::teams::get_top_teams_standings;
use crate::ui::component::goto::{goto_driver, goto_team};
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, Scroll, SizedBox};
use druid::Widget;

pub fn build_screen() -> impl Widget<AppState> {
    let mut left_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    left_column.add_child(Label::new("Top 3 drivers standings").with_text_size(20.0));
    left_column.add_spacer(5.0);
    left_column.add_flex_child(
        SizedBox::new(
            Scroll::new(make_table(
                vec![
                    "#".to_string(),
                    "Driver Name".to_string(),
                    "Points".to_string(),
                ],
                get_top_driver_standings(None),
                vec![(1, goto_driver())],
            ))
            .vertical(),
        )
        .height(500.0),
        1.0,
    );

    let mut right_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    right_column.add_child(Label::new("Top 3 teams standings").with_text_size(20.0));
    right_column.add_spacer(5.0);
    right_column.add_flex_child(
        SizedBox::new(
            Scroll::new(make_table(
                vec![
                    "#".to_string(),
                    "Team Name".to_string(),
                    "Points".to_string(),
                ],
                get_top_teams_standings(None),
                vec![(1, goto_team())],
            ))
            .vertical(),
        )
        .height(500.0),
        1.0,
    );

    let layout = Flex::row()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_flex_child(left_column, 1.0)
        .with_spacer(40.0)
        .with_flex_child(right_column, 1.0);

    Flex::column().with_spacer(20.0).with_child(layout)
}
