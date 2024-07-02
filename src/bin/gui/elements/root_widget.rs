use druid::{
    widget::{self, Button, Flex, Label, Scroll},
    EventCtx, Target, UnitPoint, Widget, WidgetExt,
};

use crate::gui::app_state::{AppState, WindowSection};

use super::{bucket_management_menu::make_bucket_management, controller, package_search_menu};

pub fn root_widget() -> impl Widget<AppState> {
    let logger_output = Scroll::new(
        Label::dynamic(|data: &AppState, _| data.console_buff.get_contents())
            .with_line_break_mode(druid::widget::LineBreaking::WordWrap),
    )
    .vertical();

    Flex::column()
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("search package").on_click(|_, data: &mut AppState, _| {
                        data.cur_window = WindowSection::PackageSearch
                    }),
                )
                .with_child(Button::new("bucket management").on_click(
                    |ctx: &mut EventCtx, data: &mut AppState, _| {
                        ctx.submit_command(controller::UPDATE_BUCKETS.to(Target::Global));
                        data.cur_window = WindowSection::BucketManagement;
                    },
                ))
                .align_horizontal(UnitPoint::LEFT)
                .align_vertical(UnitPoint::TOP),
        )
        .with_flex_child(
            widget::ViewSwitcher::new(
                |data: &AppState, _| data.cur_window.clone(),
                |section: &WindowSection, _, _| match section {
                    WindowSection::PackageSearch => {
                        Box::new(package_search_menu::make_package_search())
                    }
                    WindowSection::BucketManagement => Box::new(make_bucket_management()),
                },
            ),
            0.8,
        )
        .with_flex_child(logger_output, 0.2)
        .controller(super::controller::AppController)
}
