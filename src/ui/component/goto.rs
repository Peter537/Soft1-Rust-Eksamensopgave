use crate::ui::AppState;
use druid::EventCtx;

use crate::ui::Screen::{DriverScreen, TeamScreen};

use crate::database::teams::get_team_id_by_short_name;
use crate::database::driver::get_driver_id_by_fullname;

pub fn goto_driver() -> Box<dyn Fn(&str) -> Box<dyn Fn(&mut EventCtx, &mut AppState)>> {
    Box::new(|driver: &str| {
        let driver = driver.to_string();
        Box::new(move |_ctx: &mut EventCtx, _data: &mut AppState| {
            println!("Clicked driver: {}", driver);

            let driver_id = get_driver_id_by_fullname(&driver).unwrap_or(-1);

            _data.current_screen = DriverScreen {
                driver_id,
            };
            
            _ctx.request_update();
        })
    })
}

pub fn goto_team() -> Box<dyn Fn(&str) -> Box<dyn Fn(&mut EventCtx, &mut AppState)>> {
    Box::new(|team: &str| {
        let team = team.to_string();
        
        Box::new(move |_ctx: &mut EventCtx, _data: &mut AppState| {
            println!("Clicked team: {}", team);

            let team_id = get_team_id_by_short_name(&team).unwrap_or(-1);

            _data.current_screen = TeamScreen {
                team_id,
            };

            _ctx.request_update();
        })
    })
}
