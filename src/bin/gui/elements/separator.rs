use druid::{widget::SizedBox, Color, Data, Insets, Widget, WidgetExt};

pub fn make_separator<T: Data>() -> impl Widget<T> {
    SizedBox::empty()
        .height(2.0)
        .expand_width()
        .background(Color::GRAY)
        .padding(Insets::uniform_xy(28.0, 3.0))
}
