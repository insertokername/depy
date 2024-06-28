use depy::bucket;
use druid::{
    widget::{Button, Flex, Label, List, Scroll, TextBox},
    Command, EventCtx, Target, UnitPoint, Widget, WidgetExt,
};

use crate::gui::app_state::AppState;

use super::controller;

fn bucket_list_elem()->  impl Widget<(String, String)>{
     Label::dynamic(|data: &(String, String), _| format!("Bucket: {}  Name: {}",data.0, data.1))
}

pub fn make_bucket_management() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Button::new("refresh buckets").on_click(|_, data: &mut AppState, _| data.bucket_list=bucket::list_buckets().unwrap().into()))
        .with_flex_child(List::new(||bucket_list_elem()).lens(AppState::bucket_list), 1.0)
        .with_child(
            Flex::row()
                .with_child(
                    TextBox::new()
                        .with_placeholder("bucket name")
                        .lens(AppState::add_bucket_name_field),
                )
                .with_child(
                    TextBox::new()
                        .with_placeholder("bucket url")
                        .lens(AppState::add_bucket_url_field),
                )
                .with_child(
                    Flex::column().with_child(Button::new("add bucket").on_click(|ctx: &mut EventCtx, _, _| {
                        ctx.submit_command(Command::new(controller::ADD_BUCKET, (), Target::Global))
                    }))
                    .with_child(Button::new("remove bucket").on_click(|ctx: &mut EventCtx, _, _| {
                        ctx.submit_command(Command::new(controller::REMOVE_BUCKET, (), Target::Global))
                    })),
                ),
        )
        .align_horizontal(UnitPoint::CENTER)
}
