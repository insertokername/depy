use druid::{
    widget::{Button, Container, Flex, Label, List, Scroll, TextBox},
    Command, EventCtx, Insets, Target, Widget, WidgetExt,
};

use crate::gui::app_state::AppState;

use super::controller;

fn bucket_list_elem() -> impl Widget<(String, String)> {
    Container::new(
        Label::dynamic(|data: &(String, String), _| {
            format!("Bucket: {}  Name: {}", data.0, data.1)
        })
        .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
        .padding(Insets::uniform_xy(0.0, 2.0)),
    )
    .rounded(3.0)
    .border(druid::theme::BORDER_LIGHT, 1.0)
    .padding(Insets::uniform_xy(0.0, 7.0))
}

pub fn make_bucket_management() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(
                    TextBox::new()
                        .with_placeholder("bucket name")
                        .lens(AppState::add_bucket_name_field)
                        .fix_width(130.0)
                        .padding(Insets::uniform_xy(4.0, 0.0)),
                )
                .with_child(
                    TextBox::new()
                        .with_placeholder("bucket url")
                        .lens(AppState::add_bucket_url_field)
                        .fix_width(130.0)
                        .padding(Insets::uniform_xy(4.0, 0.0)),
                )
                .with_child(
                    Flex::column()
                        .with_child(
                            Button::new("add bucket")
                                .on_click(|ctx: &mut EventCtx, _, _| {
                                    ctx.submit_command(Command::new(
                                        controller::ADD_BUCKET,
                                        (),
                                        Target::Global,
                                    ))
                                })
                                .padding(Insets::uniform(2.1)),
                        )
                        .with_child(
                            Button::new("remove bucket")
                                .on_click(|ctx: &mut EventCtx, _, _| {
                                    ctx.submit_command(Command::new(
                                        controller::REMOVE_BUCKET,
                                        (),
                                        Target::Global,
                                    ))
                                })
                                .padding(Insets::uniform(2.1)),
                        ),
                ),
        )
        .with_flex_child(
            Scroll::new(List::new(|| bucket_list_elem()).lens(AppState::bucket_list))
                .vertical()
                .padding(Insets::new(0.0, 10.0, 0.0, 0.0)),
            1.0,
        )
}
