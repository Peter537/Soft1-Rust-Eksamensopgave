use super::component::goto::goto_race;
use crate::database::config::{get_current_date, update_current_date};
use crate::database::driver::get_top_driver_standings;
use crate::database::race::{get_next_race, get_race_list};
use crate::database::teams::{get_own_team_standing, get_top_teams_standings};
use crate::ui::component::goto::{goto_driver, goto_team};
use crate::ui::component::table::make_table;
use crate::ui::Screen::RaceScreen;
use crate::ui::{AppState, SET_CURRENT_DATE};
use chrono::NaiveDate;
use druid::widget::{
    Button, Controller, CrossAxisAlignment, Flex, Label, MainAxisAlignment, Scroll, SizedBox,
};
use druid::{Command, Env, LifeCycle, LifeCycleCtx, Target, Widget, WidgetExt};

struct InitDateController;

impl<W: Widget<AppState>> Controller<AppState, W> for InitDateController {
    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            let current_date = get_current_date().unwrap().to_string();
            ctx.submit_command(Command::new(SET_CURRENT_DATE, current_date, Target::Auto));
        }
        child.lifecycle(ctx, event, _data, _env);
    }
}

pub fn build_screen() -> impl Widget<AppState> {
    let no_races_left_string = "Next Year".to_string();
    let next_race_day: String = match get_next_race() {
        Some(race) => NaiveDate::parse_from_str(&race.date, "%Y-%m-%d")
            .unwrap()
            .to_string(),
        None => no_races_left_string.clone(),
    };

    let new_action_button =
        Button::new("Next Action").on_click(move |_ctx, _data: &mut AppState, _env| {
            let next_race = get_next_race().unwrap();
            let next_race_day = NaiveDate::parse_from_str(&next_race.date, "%Y-%m-%d").unwrap();

            if get_current_date().unwrap() == next_race_day {
                _data.current_screen = RaceScreen {
                    race_id: next_race.id.clone(),
                };
                _ctx.request_update();
            } else {
                update_current_date(&next_race_day);
                _data.current_date = next_race_day.to_string();
                _ctx.request_update();
            }
        });

    let mut column1 = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    column1.add_child(Label::new("Race List").with_text_size(20.0));
    column1.add_spacer(5.0);
    column1.add_child(
        SizedBox::new(
            Scroll::new(make_table(
                vec![
                    "Date".to_string(),
                    "Race".to_string(),
                    "Winner".to_string(),
                    "MyTeam Positions".to_string(),
                ],
                get_race_list(),
                vec![(1, goto_race()), (2, goto_driver())],
            ))
            .vertical(),
        )
        .height(500.0),
    );
    column1.add_spacer(10.0);

    let (team_name, drivers, total_points) =
        get_own_team_standing().unwrap_or(("".to_string(), vec![], 0));

    let mut column2 = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);

    column2.add_child(Label::new(|data: &AppState, _env: &_| {
        format!("Current Date: {}", data.current_date)
    }));
    column2.add_child(Label::new(
        "Next Race Date: ".to_owned() + &next_race_day.to_string(),
    ));
    if next_race_day != no_races_left_string {
        column2.add_child(new_action_button);
    }

    column2.add_spacer(10.0);
    column2.add_child(Label::new("My Team").with_text_size(20.0));

    let col2_container = Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new(format!("Team: {}", team_name)).with_text_size(16.0))
        .with_spacer(5.0)
        .with_child(Label::new(format!("Drivers: {:?}", drivers.join(", "))).with_text_size(16.0))
        .with_spacer(5.0)
        .with_child(Label::new(format!("Total Points: {}", total_points)).with_text_size(16.0))
        .border(druid::theme::BORDER_DARK, 1.0);

    column2.add_flex_child(col2_container, 1.0);
    column2.add_spacer(10.0);

    column2.add_child(Label::new("Top 3 drivers standings").with_text_size(20.0));
    column2.add_spacer(5.0);
    column2.add_child(make_table(
        vec![
            "#".to_string(),
            "Driver Name".to_string(),
            "Points".to_string(),
        ],
        get_top_driver_standings(Some(3)),
        vec![(1, goto_driver())],
    ));
    column2.add_spacer(10.0);

    column2.add_child(Label::new("Top 3 team standings").with_text_size(20.0));
    column2.add_spacer(5.0);
    column2.add_child(make_table(
        vec![
            "#".to_string(),
            "Team Name".to_string(),
            "Points".to_string(),
        ],
        get_top_teams_standings(Some(3)),
        vec![(1, goto_team())],
    ));
    //////////////////////////////

    let layout = Flex::row()
        .main_axis_alignment(MainAxisAlignment::SpaceAround)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_flex_child(column1, 1.0)
        .with_flex_child(column2, 1.0);

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacer(20.0)
        .with_child(layout)
        .controller(InitDateController)
        .boxed()
}
