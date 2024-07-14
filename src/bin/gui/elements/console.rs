use std::f64::INFINITY;

use crate::gui::app_state::LogBufferState;
use druid::{
    theme::BORDER_DARK,
    widget::{Button, Container, Either, Flex, Label, Scroll},
    BoxConstraints, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Selector, Size, Target, UpdateCtx, Widget, WidgetExt,
};

const SCROLL_BOTTOM: Selector<()> = Selector::new("scroll-bottom");
const RELEASE_SCROLL: Selector<()> = Selector::new("release_scroll");
pub struct ConsoleScroll {
    child: Scroll<LogBufferState, Label<LogBufferState>>,
}

impl ConsoleScroll {
    pub fn new(widget: Label<LogBufferState>) -> Self {
        ConsoleScroll {
            child: Scroll::new(widget).vertical(),
        }
    }
}

impl Widget<LogBufferState> for ConsoleScroll {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut LogBufferState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) => {
                if let Some(()) = cmd.get(SCROLL_BOTTOM) {
                    data.is_bottom = true;
                }
                if let Some(()) = cmd.get(RELEASE_SCROLL) {
                    data.is_bottom = false;
                }
            }
            Event::WindowConnected => {
                ctx.request_anim_frame();
            }
            Event::AnimFrame(_) => {
                if data.is_bottom {
                    self.child
                        .scroll_by(ctx, (INFINITY, INFINITY).into());
                    ctx.submit_command(RELEASE_SCROLL.to(Target::Global));
                }
                ctx.request_paint();
                ctx.request_anim_frame();
            }
            _ => (),
        }
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &LogBufferState,
        env: &Env,
    ) {
        self.child
            .lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &LogBufferState,
        data: &LogBufferState,
        env: &Env,
    ) {
        if !old_data
            .log_buffer
            .same(&data.log_buffer)
        {
            ctx.submit_command(SCROLL_BOTTOM.to(Target::Global));
        }
        self.child
            .update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &LogBufferState,
        env: &Env,
    ) -> Size {
        self.child.layout(ctx, bc, data, env)
    }

    fn paint(
        &mut self,
        ctx: &mut PaintCtx,
        data: &LogBufferState,
        env: &Env,
    ) {
        self.child.paint(ctx, data, env);
    }
}

pub fn make_console() -> impl Widget<LogBufferState> {
    Container::new(
        Flex::column()
            .with_child(Either::new(
                |data: &LogBufferState, _| {
                    data.log_buffer
                        .get_contents()
                        .is_empty()
                },
                Flex::column(),
                Button::new("X")
                    .on_click(|_, data: &mut LogBufferState, _| {
                        data.log_buffer
                            .mutate_contents(|content| content.clear())
                    })
                    .padding(Insets::uniform(2.1))
                    .align_right(),
            ))
            .with_flex_child(
                ConsoleScroll::new(
                    Label::dynamic(|data: &LogBufferState, _| data.log_buffer.get_contents())
                        .with_line_break_mode(druid::widget::LineBreaking::WordWrap),
                ),
                1.0,
            ),
    )
    .border(BORDER_DARK, 3.0)
}
