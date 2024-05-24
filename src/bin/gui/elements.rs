use druid::theme::*;
use druid::widget::{Container, Flex, Label, Scroll, TextBox};
use druid::{Color, Widget, WidgetExt};

use super::AppState;

pub fn build_root_widget() -> impl Widget<AppState> {
    let search_box = TextBox::new()
        .with_text_size(TEXT_SIZE_LARGE)
        .with_placeholder("Who are we greeting?")
        .lens(AppState::search_term);
    let search_bar = Container::new(
        Container::new(search_box)
            .rounded(2.0)
            .expand_width()
            .padding(10.0),
    )
    .expand_width()
    .border(Color::RED, 1.0);

    let list = {
        let mut col = Flex::column();
        let cols = 30;

        for _ in 0..cols {
            col.add_child(Label::new("20").with_text_size(TEXT_SIZE_LARGE));
        }

        Scroll::new(Container::new(col).border(Color::RED, 1.0).padding(10.0)).expand_width()
    };

    Flex::column()
        .with_child(search_bar)
        .with_flex_child(list, 1.0)
}
