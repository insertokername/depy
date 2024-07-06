use druid::{
    theme::{TEXT_SIZE_LARGE, TEXT_SIZE_NORMAL},
    widget::{Button, Flex, Label, TextBox, ViewSwitcher},
    Command, EventCtx, Lens, Target, Widget, WidgetExt,
};

use crate::gui::app_state::{InstalledPackageState, InstalledPackageWrapper};

use super::controller;

struct PackageVersionLens;
impl Lens<InstalledPackageWrapper, String> for PackageVersionLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &InstalledPackageWrapper, f: F) -> V {
        f(&data.package.version)
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(
        &self,
        data: &mut InstalledPackageWrapper,
        f: F,
    ) -> V {
        f(&mut data.package.version)
    }
}

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
        .with_child(ViewSwitcher::new(
            |data: &InstalledPackageWrapper, _| data.visual_package_state.clone(),
            |cur_state: &InstalledPackageState, _, _| {
                Box::new(match cur_state {
                    InstalledPackageState::AddPackage => Flex::row().with_child(
                        Button::new("add").on_click(|_, data: &mut InstalledPackageWrapper, _| {
                            data.visual_package_state = InstalledPackageState::SelectVersion
                        }),
                    ),
                    InstalledPackageState::SelectVersion => Flex::row()
                        .with_child(TextBox::new().lens(PackageVersionLens))
                        .with_child(Button::new("install").on_click(
                            |ctx: &mut EventCtx, data: &mut InstalledPackageWrapper, _| {
                                ctx.submit_command(Command::new(
                                    controller::ADD_PACKAGE,
                                    data.package.clone(),
                                    Target::Global,
                                ))
                            },
                        )),
                    InstalledPackageState::Remove => {
                        Flex::column().with_child(Button::new("remove").on_click(
                            |ctx: &mut EventCtx, data: &mut InstalledPackageWrapper, _| {
                                ctx.submit_command(Command::new(
                                    controller::REMOVE_PACKAGE,
                                    data.package.clone(),
                                    Target::Global,
                                ))
                            },
                        ))
                    }
                })
            },
        ))
        .expand_width()
        .padding((30.0, 0.0))
}
