use druid::{
    widget::{Button, Container, Either, Flex, Label},
    Color, Widget, WidgetExt,
};

use crate::ui::AppState;

pub fn build_modal() -> impl Widget<AppState> {
    Either::new(
        |data: &AppState, _env| data.show_modal,
        Container::new(
            Flex::column()
                .with_child(Label::new("Choose Career"))
                .with_spacer(10.0)
                .with_child(
                    Button::new("Make Choice").on_click(|_ctx, data: &mut AppState, _env| {
                        data.show_modal = false;
                    }),
                ),
        )
        .padding(60.0)
        .background(Color::rgb8(30, 30, 30))
        .rounded(10.0)
        .center(),
        Container::new(druid::widget::SizedBox::empty()),
    )
}
