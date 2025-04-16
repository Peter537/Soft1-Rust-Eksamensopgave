use crate::ui::AppState;
use druid::EventCtx;

use crate::ui::Screen::{DriverScreen, TeamScreen};

pub fn goto_driver() -> Box<dyn Fn(&str) -> Box<dyn Fn(&mut EventCtx, &mut AppState)>> {

    Box::new(|team: &str| {
        let driver = team.to_string(); // Capture the team name
        Box::new(move |_ctx: &mut EventCtx, _data: &mut AppState| {
            println!("Clicked driver: {}", driver);

            if driver.contains(" ") {
                // split on " " and take the first part
                let driver = driver.split(" ").next().unwrap_or(&driver).to_string();

                _data.current_screen = DriverScreen {driver_name: driver.to_string()};
            } else {
                _data.current_screen = DriverScreen {driver_name: driver.to_string()};
            }
            _ctx.request_update();
        })
    })
}

pub fn goto_team() -> Box<dyn Fn(&str) -> Box<dyn Fn(&mut EventCtx, &mut AppState)>> {

    Box::new(|team: &str| {
        let team = team.to_string(); // Capture the team name
        Box::new(move |_ctx: &mut EventCtx, _data: &mut AppState| {
            println!("Clicked team: {}", team);

            if team.contains(" ") {
                let team = team.replace(" ", "_");

                _data.current_screen = TeamScreen {team_name: team.to_string()};
            } else {
                _data.current_screen = TeamScreen {team_name: team.to_string()};
            }
            _ctx.request_update();
        })
    })
}
