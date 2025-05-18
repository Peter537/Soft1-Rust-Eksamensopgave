use super::AppState;
use crate::backend::race::start_race;
use crate::database::circuit::get_circuit_by_id;
use crate::database::country::get_country_image_path;
use crate::database::race::{get_race_results, get_season_schedule_by_id, is_next_race};
use crate::ui::component::goto::{goto_driver, goto_team};
use crate::ui::component::table::make_table;
use crate::ui::Screen::RaceScreen;
use crate::ui::ViewSwitcher;
use crate::util::image_loader::{get_circuit, get_country};
use crate::util::time::format_time;
use chrono::Utc;
use druid::widget::{
    Button, Container, CrossAxisAlignment, Flex, Label, MainAxisAlignment, Scroll, SizedBox,
};
use druid::{Color, Env, Widget, WidgetExt};

pub fn build_screen(race_id: u16) -> impl Widget<AppState> {
    let circuit_data = get_circuit_by_id(&race_id).unwrap();
    let circuit_image = get_circuit(&circuit_data.image_path)
        .fix_width(400.0)
        .fix_height(300.0);
    let circuit_country_image =
        get_country(&get_country_image_path(&circuit_data.country_id).unwrap());

    let circuit_info: Container<AppState> = Container::new(
        Flex::column()
            .with_child(Label::new(format!("Circuit: {}", circuit_data.name)))
            .with_child(Label::new(format!("Location: {}", circuit_data.city)))
            .with_child(Label::new(format!("Length: {} km", circuit_data.length_km)))
            .with_child(Label::new(format!("Laps: {}", circuit_data.lap_amount))),
    )
    .padding(10.0)
    .border(Color::grey(0.5), 1.0);

    let results_switcher = ViewSwitcher::new(
        |data: &AppState, _env: &Env| data.last_race_update_time.clone(),
        move |_key, _data: &AppState, _env| {
            if get_season_schedule_by_id(&race_id).unwrap().status == "Finished" {
                let results = get_race_results(&race_id);
                let rows: Vec<Vec<String>> = results
                    .into_iter()
                    .map(|r| {
                        vec![
                            r.position.to_string(),
                            r.driver_number.to_string(),
                            r.driver_name,
                            r.team,
                            r.points.to_string(),
                            format_time(r.total_time_ms),
                        ]
                    })
                    .collect();
                Box::new(
                    Flex::column()
                        .with_child(Label::new("Race Results"))
                        .with_spacer(10.0)
                        .with_child(
                            SizedBox::new(
                                Scroll::new(make_table(
                                    vec![
                                        "Position".into(),
                                        "DriverNumber".into(),
                                        "DriverName".into(),
                                        "Team".into(),
                                        "Points".into(),
                                        "Total Time".into(),
                                    ],
                                    rows,
                                    vec![(2, goto_driver()), (3, goto_team())],
                                ))
                                .vertical(),
                            )
                            .height(500.0),
                        ),
                )
            } else if is_next_race(&race_id) {
                let btn =
                    Button::new("Start Race").on_click(move |ctx, data: &mut AppState, _env| {
                        start_race(race_id);
                        data.last_race_update_time = Utc::now().to_string();
                        data.current_screen = RaceScreen { race_id };
                        ctx.request_update();
                    });
                Box::new(btn)
            } else {
                Box::new(Label::new("This isn't the next race."))
            }
        },
    );

    let column1 = Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_spacer(10.0)
        .with_child(results_switcher);

    let mut column2 = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    column2.add_child(circuit_image);
    column2.add_child(circuit_country_image);
    column2.add_child(circuit_info);

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(
            Flex::row()
                .main_axis_alignment(MainAxisAlignment::Center)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .must_fill_main_axis(true)
                .with_flex_child(column1, 1.0)
                .with_spacer(40.0)
                .with_flex_child(column2, 1.0),
            1.0,
        )
}
