use druid::widget::{Button, Container, CrossAxisAlignment, Flex, Image, Label, MainAxisAlignment};
use druid::{Color, ImageBuf, Widget, WidgetExt};

use druid::piet::ImageFormat;

use image::load_from_memory;

use super::AppState;

pub fn build_screen() -> impl Widget<AppState> {
    let home_button = Button::new("Home").on_click(|_ctx, _data: &mut AppState, _env| {
        // Logic for home button
        _data.current_screen = super::Screen::Main; // Go back to main screen
        _ctx.request_update();
        println!("Home button clicked!");
    });

    let (img, circuit_info) = circuit_info();

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
                            .with_spacer(20.0),
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
    Container::new(
        Flex::column()
            .with_child(Label::new("Result"))
            .with_spacer(20.0)
            .with_child(Label::new("Position: 1"))
            .with_spacer(20.0)
            .with_child(Label::new("DriverNumber: 1"))
            .with_spacer(20.0)
            .with_child(Label::new("DriverName: Driver 1"))
            .with_spacer(20.0)
            .with_child(Label::new("Team: Team 1"))
            .with_spacer(20.0)
            .with_child(Label::new("Lap Time: 1:30.123"))
            .with_spacer(20.0)
            .with_child(Label::new("Points: 25"))
            .with_spacer(20.0),
    )
    .padding(10.0)
    .border(Color::grey(0.5), 1.0)
}

fn circuit_info() -> (impl Widget<AppState>, impl Widget<AppState>) {
    let image_bytes = include_bytes!("./circuit.png"); // should take from appsdata or similar, e.g SQLlite

    let dyn_image = load_from_memory(image_bytes).expect("Failed to decode image");

    let rgba_image = dyn_image.to_rgba8();
    let (width, height) = rgba_image.dimensions();

    let image_buf = ImageBuf::from_raw(
        rgba_image.into_raw(),
        ImageFormat::RgbaSeparate,
        width as usize,
        height as usize,
    );

    let image_widget = Image::new(image_buf).fix_size(200.0, 200.0);

    let circuit_info: Container<AppState> = Container::new(
        Flex::column()
            .with_child(Label::new("Circuit Name: Circuit 1"))
            .with_spacer(20.0)
            .with_child(Label::new("Location: Location 1"))
            .with_spacer(20.0)
            .with_child(Label::new("Length: 5.5 km"))
            .with_spacer(20.0)
            .with_child(Label::new("Laps: 50"))
            .with_spacer(20.0),
    )
    .padding(10.0)
    .border(Color::grey(0.5), 1.0);

    (image_widget, circuit_info)
}
