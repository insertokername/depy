use druid::{
    widget::{Button, Flex},
    Command, EventCtx, Insets, Target, UnitPoint, Widget, WidgetExt,
};

use crate::gui::{app_state::AppState, elements::controller};

pub fn make_garbage_clean() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("Cleanup Packages")
                        .on_click(|ctx: &mut EventCtx, _, _| {
                            ctx.submit_command(Command::new(
                                controller::CLEAN_DEPY,
                                false,
                                Target::Global,
                            ))
                        })
                        .padding(Insets::uniform(2.1)),
                )
                .with_child(
                    Button::new("Uninstall Depy")
                        .on_click(|ctx: &mut EventCtx, _, _| {
                            ctx.submit_command(Command::new(
                                controller::UNINSTALL_DEPY,
                                false,
                                Target::Global,
                            ))
                        })
                        .padding(Insets::uniform(2.1)),
                )
                .align_vertical(UnitPoint::new(0.5, 0.0)),
        )
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("Force Cleanup")
                        .on_click(|ctx: &mut EventCtx, _, _| {
                            ctx.submit_command(Command::new(
                                controller::CLEAN_DEPY,
                                true,
                                Target::Global,
                            ))
                        })
                        .padding(Insets::uniform(2.1)),
                )
                .with_child(
                    Button::new("Force Uninstall")
                        .on_click(|ctx: &mut EventCtx, _, _| {
                            ctx.submit_command(Command::new(
                                controller::UNINSTALL_DEPY,
                                true,
                                Target::Global,
                            ))
                        })
                        .padding(Insets::uniform(2.1)),
                )
                .align_vertical(UnitPoint::new(0.5, 0.0)),
        )
}
