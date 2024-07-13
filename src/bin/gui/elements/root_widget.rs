use druid::{
    widget::{Either, Flex, Label, Split, ViewSwitcher},
    EventCtx, Insets, LifeCycleCtx, Target, UnitPoint, Widget, WidgetExt,
};

use crate::gui::app_state::{AppState, WindowSection};

use super::{
    bucket_management_menu::{self},
    console_widget::make_console,
    controller,
    garbage_clean_menu::{self},
    package_search_menu,
    precent_height_widget::PercentHeightWidget,
    separator::make_separator,
};

pub fn root_widget() -> impl Widget<AppState> {
    let split = Split::columns(
        Flex::column()
            .with_child(
                Label::new("search package")
                    .with_text_size(druid::theme::TEXT_SIZE_NORMAL)
                    .on_click(|_, data: &mut AppState, _| {
                        data.cur_window = WindowSection::PackageSearch
                    })
                    .align_left()
                    .padding(Insets::uniform_xy(8.0, 0.0)),
            )
            .with_child(make_separator())
            .with_child(
                Label::new("bucket management")
                    .with_text_size(druid::theme::TEXT_SIZE_NORMAL)
                    .on_click(|ctx: &mut EventCtx, data: &mut AppState, _| {
                        ctx.submit_command(controller::UPDATE_BUCKETS.to(Target::Global));
                        data.cur_window = WindowSection::BucketManagement;
                    })
                    .align_left()
                    .padding(Insets::uniform_xy(8.0, 0.0)),
            )
            .with_child(make_separator())
            .with_child(
                Label::new("garbage clean")
                    .with_text_size(druid::theme::TEXT_SIZE_NORMAL)
                    .on_click(|_, data: &mut AppState, _| {
                        data.cur_window = WindowSection::GarbageClean;
                    })
                    .align_left()
                    .padding(Insets::uniform_xy(8.0, 0.0)),
            )
            .align_horizontal(UnitPoint::LEFT)
            .align_vertical(UnitPoint::TOP)
            .padding(Insets::new(0.0, 15.0, 0.0, 0.0)),
        ViewSwitcher::new(
            |data: &AppState, _| data.cur_window.clone(),
            |section: &WindowSection, _, _| match section {
                WindowSection::PackageSearch => {
                    Box::new(package_search_menu::make_package_search())
                }
                WindowSection::BucketManagement => {
                    Box::new(bucket_management_menu::make_bucket_management())
                }
                WindowSection::GarbageClean => Box::new(garbage_clean_menu::make_garbage_clean()),
            },
        )
        .padding(Insets::uniform_xy(5.0, 15.0)),
    )
    .solid_bar(true)
    .split_point(0.3)
    .bar_size(3.0);

    Flex::column()
        .with_flex_child(split, 1.0)
        .with_child(Either::new(
            |data: &AppState, _| data.console_buff.log_buffer.get_contents().is_empty(),
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
