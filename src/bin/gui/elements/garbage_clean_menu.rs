use druid::{
    widget::{Button, Flex},
    EventCtx, Target, UnitPoint, Widget, WidgetExt,
};

use crate::gui::app_state::AppState;

use super::controller;

pub fn make_garbage_clean() -> impl Widget<AppState> {
    Flex::row()
        .with_child(
            Button::new("Cleanup Packages").on_click(|ctx: &mut EventCtx, _, _| {
                ctx.submit_command(controller::CLEAN_DEPY.to(Target::Global))
            }),
        )
        .with_child(
            Button::new("Uninstall Depy").on_click(|ctx: &mut EventCtx, _, _| {
                ctx.submit_command(controller::UNINSTALL_DEPY.to(Target::Global))
            }),
        )
        .align_vertical(UnitPoint::new(0.5, 0.0))
}
