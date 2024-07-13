use druid::{widget::SizedBox, Color, Data, Insets, Widget, WidgetExt};

pub fn make_separator<T: Data>() -> impl Widget<T> {
    SizedBox::empty()
        .height(2.0)
        .expand_width()
        .background(Color::GRAY)
        .padding(Insets::new(0.0, 5.0, 40.0, 5.0))
}
