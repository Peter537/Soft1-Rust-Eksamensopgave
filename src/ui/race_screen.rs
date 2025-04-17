use druid::widget::{Button, Container, CrossAxisAlignment, Flex, Label, MainAxisAlignment};
use druid::{Color, Widget, WidgetExt};

use super::AppState;
use super::Screen::{Main, MainGameScreen};

use super::component::table::make_table;
use crate::backend::race::start_race;
use crate::database::race::get_circuit_info;
use crate::model::circuit::CircuitInfo;
use crate::util::image_loader::get_circuit;

pub fn build_screen(race_id: i32) -> impl Widget<AppState> {
    println!("race_id: {}", &race_id.to_string());

    let home_button = Button::new("Home").on_click(|_ctx, _data: &mut AppState, _env| {
        // Logic for home button
        _data.current_screen = Main; // Go back to main screen
        _ctx.request_update();
        println!("Home button clicked!");
    });

    let (img, circuit_info) = circuit_info(&race_id);

    let start_race_button =
        Button::new("Start Race").on_click(move |_ctx, _data: &mut AppState, _env| {
            start_race(race_id);
            _data.current_screen = MainGameScreen;
            _ctx.request_update();
            println!("Start Race button clicked!");
        });

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(Label::new("Race Screen"))
        .with_spacer(20.0)
        .with_flex_child(
            Container::new(
                Flex::row()
                    .main_axis_alignment(MainAxisAlignment::Center)
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .must_fill_main_axis(true)
                    .with_flex_child(
                        Flex::column()
                            .with_spacer(20.0)
                            .with_child(race_result())
                            .with_spacer(20.0)
                            .with_child(start_race_button),
                        1.0,
                    )
                    .with_spacer(200.0)
                    .with_flex_child(
                        Flex::column()
                            .with_child(img)
                            .with_spacer(20.0)
                            .with_child(circuit_info),
                        1.0,
                    ),
            )
            .center(),
            1.0,
        )
        .with_spacer(20.0)
        .with_child(home_button)
}

fn race_result() -> impl Widget<AppState> {
    let race_cols = vec![
        "Position".to_string(),
        "DriverNumber".to_string(),
        "DriverName".to_string(),
        "Team".to_string(),
        "Lap Time".to_string(),
        "Points".to_string(),
    ];

    // make dommain for the table if empty
    let data: Vec<Vec<String>> = race_cols
        .iter()
        .map(|_| vec!["".to_string(); race_cols.len()])
        .collect();

    let race_table = make_table(race_cols, data, vec![]);

    Flex::column()
        .with_child(Label::new("Result"))
        .with_spacer(20.0)
        .with_child(race_table)
        .with_spacer(20.0)
}

fn circuit_info(id: &i32) -> (impl Widget<AppState>, impl Widget<AppState>) {
    let circuit_data: CircuitInfo = get_circuit_info(&id).unwrap();
    let img: String = circuit_data
        .image_path
        .as_deref()
        .unwrap_or("No Image")
        .to_string();

    let circuit_info: Container<AppState> = Container::new(
        Flex::column()
            .with_child(Label::new(format!(
                "Circuit: {}",
                circuit_data.circuit_name
            )))
            .with_spacer(20.0)
            .with_child(Label::new(format!("Location: {}", circuit_data.location)))
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
