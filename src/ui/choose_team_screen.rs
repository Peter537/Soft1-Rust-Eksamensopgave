use druid::widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, Scroll};
use druid::WidgetExt;
use druid::{Env, Widget};

use super::AppState;
use super::Screen::MainGameScreen;

use crate::database::teams::{get_all_teams, save_selected_team};

pub fn build_screen() -> impl Widget<AppState> {
    let teams = get_all_teams();

    // Create balanced columns
    let mut left_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let mut right_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);

    // Manual counts for balancing
    let mut left_count = 0;
    let mut right_count = 0;

    for (full_name, short_name, drivers) in &teams {
        let team_label =
            Label::new(format!("{} ({})", full_name, &short_name)).with_text_size(16.0);

        // Build driver column manually
        let mut driver_column = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
        driver_column.add_child(team_label);
        driver_column.add_spacer(5.0);

        for (first, last) in drivers {
            let driver_label = Label::new(format!("{} {}", first, last)).with_text_size(14.0);
            driver_column.add_child(driver_label);
        }

        // Balance columns manually
        if left_count <= right_count {
            left_column.add_child(driver_column);
            left_column.add_spacer(10.0);

            left_column.add_child(Button::new("Select").on_click({
                let short_name = short_name.clone(); // clone needed because short_name is &String
                move |ctx, data: &mut AppState, _env| {
                    println!("Selected team: {}", short_name);
                    data.selected_team = Some(short_name.clone());
                    ctx.request_update();
                }
            }));

            left_column.add_spacer(15.0);
            left_count += 1;
        } else {
            right_column.add_child(driver_column);
            right_column.add_spacer(10.0);

            right_column.add_child(Button::new("Select").on_click({
                let short_name = short_name.clone();
                move |ctx, data: &mut AppState, _env| {
                    println!("Selected team: {}", short_name);
                    data.selected_team = Some(short_name.clone());
                    ctx.request_update();
                }
            }));
            right_column.add_spacer(15.0);
            right_count += 1;
        }
    }

    let two_column_inner = Flex::row()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_flex_child(left_column.expand_height(), 1.0)
        .with_spacer(60.0)
        .with_flex_child(right_column.expand_height(), 1.0);

    let scrollable_two_column_layout = Scroll::new(two_column_inner).vertical();

    let start_game_button = Button::new("Start game")
        .on_click(|ctx, data: &mut AppState, _env| {
            save_selected_team(data.selected_team.as_ref().unwrap());

            data.current_screen = MainGameScreen;
            ctx.request_update();
        })
        .disabled_if(|data: &AppState, _env| data.selected_team.is_none());

    // Display selected team
    let selected_team_label = Label::<AppState>::new(|data: &AppState, _env: &Env| {
        if let Some(ref team) = data.selected_team {
            format!("Selected team: {}", team)
        } else {
            "No team selected".to_string()
        }
    })
    .with_text_size(16.0);

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(Label::new("Choose your team").with_text_size(20.0))
        .with_spacer(20.0)
        .with_flex_child(scrollable_two_column_layout, 1.0)
        .with_spacer(20.0)
        .with_child(Label::new("Selected team:").with_text_size(16.0))
        .with_child(selected_team_label)
        .with_spacer(5.0)
        .with_child(start_game_button)
        .padding(20.0)
}
