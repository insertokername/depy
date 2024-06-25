use druid::{
    theme::{TEXT_SIZE_LARGE, TEXT_SIZE_NORMAL},
    widget::{Button, Flex, Label},
    Command, EventCtx, Target, Widget, WidgetExt,
};

use crate::gui::app_state::InstalledPackageWrapper;

use super::controller;

pub fn package_widget() -> impl Widget<InstalledPackageWrapper> {
    Flex::row()
        .main_axis_alignment(druid::widget::MainAxisAlignment::SpaceBetween)
        .with_child(
            Flex::column()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
                .with_child(
                    Label::dynamic(|data: &InstalledPackageWrapper, _| {
                        format!("{}", &data.package.name)
                    })
                    .with_text_size(TEXT_SIZE_LARGE),
                )
                .with_child(
                    Label::dynamic(|data: &InstalledPackageWrapper, _| {
                        format!("from bucket: {}", &data.package.bucket_name)
                    })
                    .with_text_size(TEXT_SIZE_NORMAL)
                    .with_line_break_mode(druid::widget::LineBreaking::WordWrap),
                ),
        )
        .with_child(Button::dynamic(|data: &InstalledPackageWrapper, _| {
            if !data.is_installed {
                "add".to_string()
            } else {
                "remove".to_string()
            }
        }))
        .on_click(
            |ctx: &mut EventCtx, data: &mut InstalledPackageWrapper, _| {
                ctx.submit_command(Command::new(
                    controller::UPDATE_PACKAGE_INSTALL_STATUS,
                    data.package.clone(),
                    Target::Global,
                ))
            },
        )
        .expand_width()
        .padding((30.0, 0.0))
}
