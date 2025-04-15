use crate::ui::AppState;
use druid::EventCtx;

use crate::ui::Screen::{DriverScreen, TeamScreen};

pub fn goto_driver() -> Box<dyn Fn(&str) -> Box<dyn Fn(&mut EventCtx, &mut AppState)>> {

    Box::new(|team: &str| {
        let driver = team.to_string(); // Capture the team name
        Box::new(move |_ctx: &mut EventCtx, _data: &mut AppState| {
            println!("Clicked driver: {}", driver);

            _data.current_screen = DriverScreen;
            _ctx.request_update();
        })
    })
}

pub fn goto_team() -> Box<dyn Fn(&str) -> Box<dyn Fn(&mut EventCtx, &mut AppState)>> {

    Box::new(|team: &str| {
        let team = team.to_string(); // Capture the team name
        Box::new(move |_ctx: &mut EventCtx, _data: &mut AppState| {
            println!("Clicked team: {}", team);

            _data.current_screen = TeamScreen;
            _ctx.request_update();
        })
    })
}
