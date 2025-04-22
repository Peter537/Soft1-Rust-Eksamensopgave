use super::component::goto::{goto_driver, goto_team};
use super::component::table::make_table;
use super::AppState;
use super::Screen::RaceScreen;
use crate::backend::race::start_race;
use crate::database::circuit::get_circuit_by_id;
use crate::database::race::{get_race_results, get_season_schedule_by_id, is_next_race};
use crate::model::circuit::Circuit;
use crate::ui::ViewSwitcher;
use crate::util::image_loader::get_circuit;
use chrono::Utc;
use druid::widget::{Button, Container, CrossAxisAlignment, Flex, Label, MainAxisAlignment};
use druid::{Color, Env, Widget, WidgetExt};

pub fn build_screen(race_id: i32) -> impl Widget<AppState> {
    // Fetch circuit info
    let (img, circuit_info) = circuit_info(&race_id);

    // This ViewSwitcher watches `last_race_update_time`.
    // Whenever that String changes, it re‑runs the builder closure.
    let results_switcher = ViewSwitcher::new(
        // child_picker: our key is the timestamp string
        |data: &AppState, _env: &Env| data.last_race_update_time.clone(),
        // child_builder: rebuilds on every timestamp bump
        move |_key, _data: &AppState, _env| {
            if get_season_schedule_by_id(race_id).unwrap().status == "Finished" {
                // ——— Build the results table ———
                let results = get_race_results(&race_id);
                let cols = vec![
                    "Position".into(),
                    "DriverNumber".into(),
                    "DriverName".into(),
                    "Team".into(),
                    "Points".into(),
                    "Total Time".into(),
                ];
                let rows: Vec<Vec<String>> = results
                    .into_iter()
                    .map(|r| {
                        vec![
                            r.position.to_string(),
                            r.driver_number.to_string(),
                            r.driver_name,
                            r.team,
                            r.points.to_string(),
                            format_time(r.total_time_ms as i32),
                        ]
                    })
                    .collect();
                let table = make_table(
                    cols.clone(),
                    rows,
                    vec![(2, goto_driver()), (3, goto_team())],
                );
                Box::new(
                    Flex::column()
                        .with_child(Label::new("Race Results"))
                        .with_spacer(10.0)
                        .with_child(table),
                )
            } else if is_next_race(race_id) {
                // ——— Re‑create the “Start Race” button fresh ———
                let btn =
                    Button::new("Start Race").on_click(move |ctx, data: &mut AppState, _env| {
                        // 1) Run your backend
                        start_race(race_id);
                        // 2) Stamp the clock so ViewSwitcher kicks in
                        data.last_race_update_time = Utc::now().to_string();
                        // 3) Keep on the same screen
                        data.current_screen = RaceScreen { race_id };
                        ctx.request_update();
                        println!("Start Race button clicked!");
                    });
                Box::new(btn)
            } else {
                Box::new(Label::new("This isn't the next race."))
            }
        },
    );

    // Assemble column1 with status label + switcher
    let column1 = Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_spacer(10.0)
        .with_child(results_switcher);

    // Column 2: Circuit information
    let mut column2 = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    column2.add_child(img);
    column2.add_spacer(20.0);
    column2.add_child(circuit_info);

    // Main layout
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

fn circuit_info(id: &i32) -> (impl Widget<AppState>, impl Widget<AppState>) {
    let circuit_data: Circuit = get_circuit_by_id(*id).unwrap();
    let img: String = circuit_data.image_path;

    let circuit_info: Container<AppState> = Container::new(
        Flex::column()
            .with_child(Label::new(format!("Circuit: {}", circuit_data.name)))
            .with_spacer(20.0)
            .with_child(Label::new(format!("Location: {}", circuit_data.city)))
            .with_spacer(20.0)
            .with_child(Label::new(format!("Length: {} km", circuit_data.length_km)))
            .with_spacer(20.0)
            .with_child(Label::new(format!("Laps: {}", circuit_data.lap_amount)))
            .with_spacer(20.0),
    )
    .padding(10.0)
    .border(Color::grey(0.5), 1.0);

    (
        get_circuit(&img).fix_width(400.0).fix_height(400.0),
        circuit_info,
    )
}

fn format_time(ms: i32) -> String {
    // return in format "H:MM:SS.mmm" for hours < 10, "HH:MM:SS.mmm" for hours >= 10
    let seconds = ms / 1000;
    let milliseconds = ms % 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let seconds = seconds % 60;
    let minutes = minutes % 60;
    format!(
        "{}:{:02}:{:02}.{:03}",
        hours, minutes, seconds, milliseconds
    )
}
