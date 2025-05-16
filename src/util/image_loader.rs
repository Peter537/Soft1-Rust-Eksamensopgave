use super::super::ui::AppState;
use crate::util::appdata::get_mod_default_path;
use druid::piet::ImageFormat;
use druid::widget::{Image, Label};
use druid::{Color, ImageBuf, Widget, WidgetExt};
use image::load_from_memory;
use std::fs;
use std::path::PathBuf;

fn get_image_at(path: PathBuf) -> Box<dyn Widget<AppState>> {
    let image_path = get_mod_default_path().join(path);
    if image_path.exists() {
        let image_bytes = fs::read(image_path).expect("Failed to read image file");

        let dyn_image = load_from_memory(&image_bytes).expect("Failed to decode image");

        let rgba_image = dyn_image.to_rgba8();
        let (width, height) = rgba_image.dimensions();

        let image_buf = ImageBuf::from_raw(
            rgba_image.into_raw(),
            ImageFormat::RgbaSeparate,
            width as usize,
            height as usize,
        );

        Box::new(Image::new(image_buf).fix_size(200.0, 200.0))
    } else {
        Box::new(Label::new("Image not found").with_text_color(Color::RED))
    }
}

pub fn get_circuit(circuit_name: &str) -> Box<dyn Widget<AppState>> {
    get_image_at(PathBuf::from("Circuits").join(circuit_name.to_owned() + ".png"))
}

pub fn get_car(car_name: &str) -> Box<dyn Widget<AppState>> {
    get_image_at(PathBuf::from("Cars").join(car_name.to_owned() + ".png"))
}

pub fn get_driver(driver_name: &str) -> Box<dyn Widget<AppState>> {
    get_image_at(PathBuf::from("Drivers").join(driver_name.to_owned() + ".png"))
}

pub fn get_team(team_name: &str) -> Box<dyn Widget<AppState>> {
    get_image_at(PathBuf::from("Teams").join(team_name.to_owned() + ".png"))
}

pub fn get_country(country_name: &str) -> Box<dyn Widget<AppState>> {
    get_image_at(PathBuf::from("Countries").join(country_name.to_owned() + ".png"))
}
