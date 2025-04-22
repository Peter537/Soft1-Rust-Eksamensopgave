use super::AppState;
use super::Screen::{ChooseTeam, MainGameScreen};
use crate::database::config::has_selected_team;
use crate::database::set_game_number;
use crate::database::teams::get_selected_team;
use crate::util::appdata;
use druid::widget::{Button, Container, Either, Flex, Label, ZStack};
use druid::{Color, UnitPoint, Vec2, Widget, WidgetExt};

pub fn build_screen() -> impl Widget<AppState> {
    let create_new_career_button =
        Button::new("Create New Career").on_click(|_ctx, _data: &mut AppState, _env| {
            appdata::create_new_career(); // Call the function to create a new career

            _data.current_screen = ChooseTeam;
            _ctx.request_update();
            println!("New career created!");
        });

    let load_save_game_button =
        Button::new("Load Saved Game").on_click(|_ctx, data: &mut AppState, _env| {
            data.show_modal = true;
            _ctx.request_update();
        });

    // Vertical layout for the widgets
    let main_content = Flex::column()
        .with_spacer(120.0) // Add some space between the buttons
        .with_child(create_new_career_button)
        .with_spacer(20.0) // Add some space between the buttons
        .with_child(load_save_game_button)
        .with_spacer(2000.0); // Add some space below so buttons on modal work.

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

pub fn build_modal() -> impl Widget<AppState> {
    Either::new(
        |data: &AppState, _env| data.show_modal,
        Container::new({
            let existing_careers = appdata::get_existing_careers();
            let mut column = Flex::column()
                .with_child(Label::new("Choose a Career").with_text_size(20.0))
                .with_spacer(10.0);

            for career_number in existing_careers {
                let label = format!("Career {}", career_number);
                let career_id = career_number; // Capture a fresh copy inside the loop

                column = column.with_child(Button::new(label.clone()).on_click(
                    move |ctx, data: &mut AppState, _env| {
                        match i32::try_from(career_id) {
                            Ok(id) => {
                                println!("Career ID: {}", career_id);
                                data.game_number = career_id.to_string(); // Update game number in AppState
                                set_game_number(id); // Update DB connection
                                if has_selected_team() == false {
                                    println!("Career {} has no team selected", career_id);
                                    return;
                                }

                                let selected_team = get_selected_team(&career_id.to_string()); // Load user's team
                                data.selected_team = selected_team; // Load user's team
                                data.current_screen = MainGameScreen; // Switch screen
                                data.show_modal = false; // Close modal
                                ctx.request_update();
                                println!("Loaded Career {}", career_id);
                            }
                            Err(_) => {
                                println!("Failed to convert career_number {} to i32", career_id);
                            }
                        }
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
