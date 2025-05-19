use super::AppState;
use crate::ui::Screen::Main;
use crate::ui::{SET_SCREEN, SHOW_ERROR};
use crate::util::appdata::create_files_if_not_exist;
use druid::widget::{Controller, Flex, Label, Spinner};
use druid::{Env, LifeCycle, LifeCycleCtx, Target, Widget, WidgetExt};
use std::thread;

pub fn build_screen() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("Loading..."))
        .with_child(Spinner::new())
        .center()
        .controller(LoadingController)
}

struct LoadingController;

impl<W: Widget<AppState>> Controller<AppState, W> for LoadingController {
    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            let sink = ctx.get_external_handle();
            thread::spawn(move || match create_files_if_not_exist() {
                Ok(_) => sink.submit_command(SET_SCREEN, Main, Target::Auto),
                Err(e) => sink.submit_command(
                    SHOW_ERROR,
                    format!("Failed to create files: {}", e),
                    Target::Auto,
                ),
            });
        }
        child.lifecycle(ctx, event, data, env);
    }
}
