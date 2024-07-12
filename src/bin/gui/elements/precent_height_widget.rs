
use crate::gui::app_state::AppState;
use druid::{
    Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Size, UpdateCtx, Widget,
};


pub struct PercentHeightWidget<W> {
    child: W,
    percentage: f64,
}

impl<W: Widget<AppState>> Widget<AppState> for PercentHeightWidget<W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        self.child.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        self.child.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        self.child.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &druid::BoxConstraints, data: &AppState, env: &Env) -> Size {
        let size = ctx.window().get_size();
        self.child.layout(ctx, &bc.shrink_max_height_to(size.height*self.percentage), data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.child.paint(ctx, data, env)
    }
}

impl<W: Widget<AppState>> PercentHeightWidget<W> {
    pub fn new(child: W, percentage: f64) -> Self {
        PercentHeightWidget { child, percentage }
    }
}
