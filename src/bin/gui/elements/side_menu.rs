use druid::{
    theme::BORDER_DARK,
    widget::{Container, Controller, Flex, Label, SizedBox},
    Data, Env, Event, EventCtx, Insets, Key, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    Target, UnitPoint, UpdateCtx, Widget, WidgetExt,
};

use crate::gui::app_state::{AppState, WindowSection};

use super::controller;

struct HoverDetector<W> {
    child: W,
    is_hovering: bool,
}

const IS_HOVERING: Key<bool> = Key::new("is-hovering");

impl<W: Widget<AppState>> Widget<AppState> for HoverDetector<W> {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if ctx.is_hot() {
            self.is_hovering = true;
            ctx.request_update();
        } else {
            self.is_hovering = false;
            ctx.request_update();
        }
        if ctx.is_disabled() {
            self.is_hovering = true;
            ctx.request_anim_frame()
        }
        self.child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        self.child
            .lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        let env = env
            .clone()
            .adding(IS_HOVERING, self.is_hovering);
        self.child
            .update(ctx, old_data, data, &env)
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.child.layout(ctx, &bc, data, env)
    }

    fn paint(
        &mut self,
        ctx: &mut PaintCtx,
        data: &AppState,
        env: &Env,
    ) {
        self.child.paint(ctx, data, env)
    }
}

impl<W: Widget<AppState> + 'static> HoverDetector<W> {
    pub fn new(child: W) -> Self {
        HoverDetector {
            child: child,
            is_hovering: false,
        }
    }
}

struct LabelHoverController;

impl Controller<AppState, Label<AppState>> for LabelHoverController {
    fn update(
        &mut self,
        child: &mut Label<AppState>,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        let mut color = env.get(druid::theme::TEXT_COLOR);

        if let Ok(is_hovering) = env.try_get(IS_HOVERING) {
            if is_hovering {
                color = env.get(druid::theme::DISABLED_TEXT_COLOR);
            }
        }
        child.set_text_color(color);

        ctx.request_layout();
        child.update(ctx, old_data, data, env)
    }
}

fn make_menu_separator<T: Data>() -> impl Widget<T> {
    SizedBox::empty()
        .height(2.0)
        .expand_width()
        .background(BORDER_DARK)
        .padding(Insets::new(0.0, 5.0, 0.0, 5.0))
}

fn make_menu_button(button_text: &str) -> impl Widget<AppState> {
    HoverDetector::new(
        Container::new(
            Flex::column()
                .with_child(
                    Label::new(button_text)
                        .with_text_size(druid::theme::TEXT_SIZE_NORMAL)
                        .controller(LabelHoverController)
                        .align_left()
                        .padding(Insets::uniform_xy(8.0, 0.0)),
                )
                .with_child(make_menu_separator()),
        )
        .expand_width(),
    )
    .padding(Insets::new(0.0, 0.0, 40.0, 0.0))
}

fn make_last_menu_button(button_text: &str) -> impl Widget<AppState> {
    HoverDetector::new(
        Container::new(
            Flex::column().with_child(
                Label::new(button_text)
                    .with_text_size(druid::theme::TEXT_SIZE_NORMAL)
                    .controller(LabelHoverController)
                    .align_left()
                    .padding(Insets::uniform_xy(8.0, 0.0)),
            ),
        )
        .expand_width(),
    )
    .padding(Insets::new(0.0, 0.0, 40.0, 0.0))
}

pub fn make_menu() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            make_menu_button("search package").on_click(|_, data: &mut AppState, _| {
                data.cur_window = WindowSection::PackageSearch
            }),
        )
        .with_child(make_menu_button("bucket management").on_click(
            |ctx: &mut EventCtx, data: &mut AppState, _| {
                ctx.submit_command(controller::UPDATE_BUCKETS.to(Target::Global));
                data.cur_window = WindowSection::BucketManagement;
            },
        ))
        .with_child(
            make_last_menu_button("garbage clean").on_click(|_, data: &mut AppState, _| {
                data.cur_window = WindowSection::GarbageClean;
            }),
        )
        .align_horizontal(UnitPoint::LEFT)
        .align_vertical(UnitPoint::TOP)
        .padding(Insets::new(0.0, 15.0, 0.0, 0.0))
}
