use std::io::Write;

use druid::{
    widget::{Button, Container, Either, Flex, Label, SizedBox, Split, ViewSwitcher},
    Color, EventCtx, Insets, LifeCycleCtx, Target, UnitPoint, Widget, WidgetExt,
};

use crate::gui::app_state::{AppState, WindowSection};

use super::{
    bucket_management_menu::make_bucket_management, console_widget::make_console, controller,
    garbage_clean_menu::make_garbage_clean, package_search_menu,
    precent_height_widget::PercentHeightWidget, separator::make_separator,
};

pub fn root_widget() -> impl Widget<AppState> {
    let split = Split::columns(
        Flex::column()
            .with_child(
                Label::new("search package")
                    .with_text_size(druid::theme::TEXT_SIZE_NORMAL)
                    .on_click(|_, data: &mut AppState, _| {
                        data.cur_window = WindowSection::PackageSearch
                    }),
            )
            .with_child(make_separator())
            .with_child(
                Label::new("bucket management")
                    .with_text_size(druid::theme::TEXT_SIZE_NORMAL)
                    .on_click(|ctx: &mut EventCtx, data: &mut AppState, _| {
                        ctx.submit_command(controller::UPDATE_BUCKETS.to(Target::Global));
                        data.cur_window = WindowSection::BucketManagement;
                    }),
            )
            .with_child(
                SizedBox::empty()
                    .height(2.0)
                    .background(Color::GRAY),
            )
            .with_child(make_separator())
            .with_child(
                Label::new("garbage clean")
                    .with_text_size(druid::theme::TEXT_SIZE_NORMAL)
                    .on_click(|_, data: &mut AppState, _| {
                        data.cur_window = WindowSection::GarbageClean;
                    }),
            )
            .align_horizontal(UnitPoint::CENTER)
            .align_vertical(UnitPoint::TOP)
            .padding(Insets::new(0.0, 20.0, 0.0, 0.0)),
        ViewSwitcher::new(
            |data: &AppState, _| data.cur_window.clone(),
            |section: &WindowSection, _, _| match section {
                WindowSection::PackageSearch => {
                    Box::new(package_search_menu::make_package_search())
                }
                WindowSection::BucketManagement => {
                    Box::new(make_bucket_management().align_vertical(UnitPoint::CENTER))
                }
                WindowSection::GarbageClean => {
                    Box::new(make_garbage_clean().align_vertical(UnitPoint::CENTER))
                }
            },
        ),
    )
    .solid_bar(true)
    .split_point(0.3)
    .bar_size(6.0);

    Flex::column()
        .with_flex_child(split, 1.0)
        .with_child(Either::new(
            |data: &AppState, _| {
                data.console_buff
                    .log_buffer
                    .get_contents()
                    .is_empty()
            },
            Flex::column(),
            PercentHeightWidget::new(make_console().lens(AppState::console_buff), 0.25),
        ))
        .on_added(|_, ctx: &mut LifeCycleCtx, _, _| {
            ctx.submit_command(controller::INITIALIZE.to(Target::Global))
        })
        .disabled_if(|data: &AppState, _| {
            data.initializing_depy | data.is_cleaning_depy | data.is_uninstalled
        })
        .controller(super::controller::AppController)
}
