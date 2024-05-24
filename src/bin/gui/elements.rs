use depy::package::{self, Package};
use druid::im::Vector;
use druid::theme::*;
use druid::widget::{Button, Container, Flex, Label, LensWrap, List, Scroll, TextBox};
use druid::{Color, Widget, WidgetExt};

use super::AppState;

fn outline(input: impl Widget<AppState> + 'static) -> impl Widget<AppState> {
    Container::new(input).border(Color::RED, 1.0).padding(10.0)
}

fn package_widget() -> impl Widget<package::Package> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_child(
            Label::dynamic(|data: &Package, _| format!("{}", &data.name))
                .with_text_size(TEXT_SIZE_LARGE),
        )
        .with_child(
            Label::dynamic(|data: &Package, _| format!("from bucket: {}", &data.bucket_name))
                .with_text_size(TEXT_SIZE_NORMAL)
                .with_line_break_mode(druid::widget::LineBreaking::WordWrap),
        )
}

pub fn build_root_widget() -> impl Widget<AppState> {
    let search_box = TextBox::new()
        .with_text_size(TEXT_SIZE_LARGE)
        .with_placeholder("Package name")
        .lens(AppState::search_term);
    let search_bar = Container::new(
        Container::new(search_box)
            .rounded(2.0)
            .expand_width()
            .padding(10.0),
    )
    .expand_width();

    let other_list = Scroll::new(LensWrap::new(
        // List::new(|| Label::dynamic(|data, _| format!("List item: {data}"))),
        List::new(|| package_widget()),
        AppState::package_list,
    ))
    .expand_width();

    let add_pkg_button = Button::new("Search Package").on_click(|_, data: &mut AppState, _| {
        data.package_list = Vector::new();
    });

    Flex::column()
        .with_child(search_bar)
        .with_child(add_pkg_button)
        .with_flex_child(other_list, 1.0)
    // .with_flex_child(list, 1.0)
}
