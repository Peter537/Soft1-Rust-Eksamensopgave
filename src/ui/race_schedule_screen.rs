use super::component::goto::{goto_driver, goto_race};
use super::AppState;
use crate::database::race::get_race_schedule_info;
use crate::ui::component::table::make_table;
use druid::widget::Flex;
use druid::Widget;

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

    Flex::column().with_spacer(20.0).with_child(make_table(
        col,
        data,
        vec![
            (2, goto_race()),
            (4, goto_driver()),
            (5, goto_driver()),
            (6, goto_driver()),
        ],
    ))
}
