use super::AppState;
use crate::database::config::has_selected_team;
use crate::database::set_game_number;
use crate::database::teams::get_selected_team;
use crate::ui::Screen::{ChooseTeam, MainGameScreen};
use crate::util::appdata;
use druid::widget::{Button, Container, Either, Flex, Label, ZStack};
use druid::{Color, UnitPoint, Vec2, Widget, WidgetExt};

pub fn build_screen() -> impl Widget<AppState> {
    let create_new_career_button =
        Button::new("Create New Career").on_click(|ctx, data: &mut AppState, _env| {
            appdata::create_new_career();
            data.current_screen = ChooseTeam;
            ctx.request_update();
        });

    let load_save_game_button =
        Button::new("Load Saved Game").on_click(|ctx, data: &mut AppState, _env| {
            data.show_modal = true;
            ctx.request_update();
        });

    let main_content = Flex::column()
        .with_spacer(120.0)
        .with_child(create_new_career_button)
        .with_spacer(20.0)
        .with_child(load_save_game_button)
        .with_spacer(2000.0);

    ZStack::new(main_content)
        .with_child(
            build_modal(),
            Vec2::new(200.0, 200.0),
            Vec2::new(0.0, 0.0),
            UnitPoint::CENTER,
            Vec2::new(0.0, 0.0),
        )
        .boxed()
}

fn build_modal() -> impl Widget<AppState> {
    Either::new(
        |data: &AppState, _env| data.show_modal,
        Container::new({
            let existing_careers = appdata::get_existing_careers();
            let mut column = Flex::column()
                .with_child(Label::new("Choose a Career").with_text_size(20.0))
                .with_spacer(10.0);

            for career_number in existing_careers {
                let label = format!("Career {}", career_number);
                let career_id = career_number;

                column = column.with_child(Button::new(label.clone()).on_click(
                    move |ctx, data: &mut AppState, _env| {
                        data.game_number = career_id.to_string();
                        set_game_number(career_id);
                        if has_selected_team() == false {
                            data.current_screen = ChooseTeam;
                        } else {
                            let selected_team = get_selected_team(&career_id.to_string());
                            data.selected_team = selected_team;
                            data.current_screen = MainGameScreen;
                        }

                        data.show_modal = false;
                        ctx.request_update();
                    },
                ));
            }

            column
                .with_spacer(30.0)
                .with_child(
                    Button::new("Cancel").on_click(|ctx, data: &mut AppState, _env| {
                        data.show_modal = false;
                        ctx.request_update();
                    }),
                )
        })
        .padding(60.0)
        .background(Color::rgb8(30, 30, 30))
        .rounded(10.0)
        .center(),
        Container::new(druid::widget::SizedBox::empty()),
    )
}
