use druid::{
    widget::{Button, Container, Either, Flex, Label},
    Color, Widget, WidgetExt,
};

use crate::ui::{main_game_screen::build_screen, AppState};
use crate::util::appdata;
use crate::database::connection::set_game_number;
use crate::database::teams::get_selected_team;
use crate::ui::Screen;

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

                column = column.with_child(
                    Button::new(label.clone()).on_click(move |ctx, data: &mut AppState, _env| {
                        match i32::try_from(career_id) {
                            Ok(id) => {
                                print!("Career ID: {}", career_id);
                                data.game_number = career_id.to_string(); // Update game number in AppState
                                set_game_number(id); // Update DB connection
                                data.selected_team = get_selected_team(&data.game_number); // Load user's team
                                build_screen(); // Load the main game screen
                                data.current_screen = Screen::MainGameScreen; // Switch screen
                                data.show_modal = false; // Close modal
                                ctx.request_update();
                                println!("Loaded Career {}", career_id);
                            }
                            Err(_) => {
                                println!(
                                    "Failed to convert career_number {} to i32",
                                    career_id
                                );
                            }
                        }
                    }),
                );
            }

            column.with_spacer(20.0).with_child(
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

