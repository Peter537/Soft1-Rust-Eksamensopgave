use druid::widget::{Button, Flex, Label};
use druid::{EventCtx, Widget};

use super::component::table::make_table;
use super::AppState;
use super::Screen::{Main, TeamScreen};

pub fn build_screen() -> impl Widget<AppState> {
    let col = vec![
        "Position".to_string(),
        "Team".to_string(),
        "Points".to_string(),
    ];

    let data = vec![
        vec!["1".to_string(), "Team A".to_string(), "100".to_string()],
        vec!["2".to_string(), "Team B".to_string(), "90".to_string()],
        vec!["3".to_string(), "Team C".to_string(), "80".to_string()],
    ];

    // Custom bnt for all cells in the second column (team names)
    let team_handler: Box<dyn Fn(&str) -> Box<dyn Fn(&mut EventCtx, &mut AppState)>> =
        Box::new(|team: &str| {
            let team = team.to_string(); // Capture the team name
            Box::new(move |_ctx: &mut EventCtx, _data: &mut AppState| {
                println!("Clicked team: {}", team);

                _data.current_screen = TeamScreen;
                _ctx.request_paint();
            })
        });

    let table = make_table(col, data, vec![(1, team_handler)]);

    // without custom button, just use this:
    // let table = make_table(col, data, vec![]);
    Flex::column()
        .with_spacer(20.0)
        .with_child(Label::new("Leaderboard Screen"))
        .with_child(
            Button::new("Back to Main").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_screen = Main;
                _ctx.request_update();
            }),
        )
        .with_spacer(40.0)
        .with_child(table)
        .with_spacer(20.0)
}
