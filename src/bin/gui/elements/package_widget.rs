use depy::package;
use druid::{
    theme::{TEXT_SIZE_LARGE, TEXT_SIZE_NORMAL},
    widget::{Button, Flex, Label},
    Widget, WidgetExt,
};

pub fn package_widget() -> impl Widget<package::Package> {
    Flex::row()
        .main_axis_alignment(druid::widget::MainAxisAlignment::SpaceBetween)
        .with_child(
            Flex::column()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
                .with_child(
                    Label::dynamic(|data: &package::Package, _| format!("{}", &data.name))
                        .with_text_size(TEXT_SIZE_LARGE),
                )
                .with_child(
                    Label::dynamic(|data: &package::Package, _| {
                        format!("from bucket: {}", &data.bucket_name)
                    })
                    .with_text_size(TEXT_SIZE_NORMAL)
                    .with_line_break_mode(druid::widget::LineBreaking::WordWrap),
                ),
        )
        .with_child(Button::new("install"))
        .expand_width()
        .padding((30.0, 0.0))
}
