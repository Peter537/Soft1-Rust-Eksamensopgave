use druid::{
    widget::{Align, Button, Container, Either, Flex, Label}, Color, Env, Event, EventCtx, TimerToken, UnitPoint, Widget, WidgetExt
};
use std::time::{Duration, Instant};

use crate::ui::AppState;

pub fn build_toast() -> impl Widget<AppState> {
    Align::new(
        UnitPoint::TOP_RIGHT, // Align the toast to the top-right corner
        Either::new(
            |data: &AppState, _env| data.show_toast,
            Container::new(
                Flex::column()
                    .with_child(Label::dynamic(|data: &AppState, _| {
                        data.toast_message.clone().unwrap_or_default()
                    }))
                    .with_spacer(10.0)
                    .with_child(Button::new("Dismiss").on_click(|_ctx, data: &mut AppState, _| {
                        data.toast_timer = None;
                        data.show_toast = false;
                    })),
            )
            .padding(10.0)
            .background(Color::rgb8(70, 70, 70))
            .rounded(10.0),
            Container::new(druid::widget::SizedBox::empty()),
        ),
    )
    .controller(ToastController)
}

struct ToastController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for ToastController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Timer(token) => {
                if let Some(toast_token) = data.toast_timer {
                    if token == &toast_token {
                        data.show_toast = false;
                        data.toast_timer = None;
                        ctx.request_paint();
                    }
                }
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}
