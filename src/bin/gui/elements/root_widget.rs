use druid::{
    widget::{Either, Flex, Split},
    LifeCycleCtx, Target, Widget, WidgetExt,
};

use crate::gui::app_state::AppState;

use super::{
    console::make_console, controller, precent_height::PercentHeightWidget,
    side_menu::make_menu, window::make_window_picker,
};

pub fn root_widget() -> impl Widget<AppState> {
    let split = Split::columns(make_menu(), make_window_picker())
        .solid_bar(true)
        .split_point(0.3)
        .bar_size(3.0);

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
