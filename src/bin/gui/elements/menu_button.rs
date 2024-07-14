use druid::{
    widget::{Container, Label}, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, Target, UpdateCtx, Widget, WidgetExt, WidgetId
};

use crate::gui::app_state::AppState;

pub struct MenuButton<W> {
    child: W,
}

impl<W: Widget<AppState>> Widget<AppState> for MenuButton<W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        if ctx.is_hot() {
            self.child.set_text_color(Color::RED);
            ctx.request_layout();
        } else {
            self.child.set_text_color(Color::BLUE);
            ctx.request_layout();
        }
        ctx.submit_command(Target::Widget(self.child.widget_id));
        self.child.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        self.child.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        self.child.update(ctx, old_data, data, env)
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.child.layout(ctx, &bc, data, env)
        // bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.child.paint(ctx, data, env)
    }
}

impl MenuButton<Label<AppState>> {
    pub fn new(child: Label<AppState>) -> Self {
        MenuButton { child: child.controller(controller) }
    }
}
