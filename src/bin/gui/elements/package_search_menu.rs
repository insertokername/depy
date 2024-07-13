use druid::{
    theme::*,
    widget::{Button, Container, Either, Flex, Label, LensWrap, List, Scroll, TextBox},
    Command, EventCtx, Target, UnitPoint, Widget, WidgetExt,
};

use crate::gui::app_state::{AppState, InstalledPackageState, InstalledPackageWrapper};

use super::controller;

pub fn make_package_search() -> impl Widget<AppState> {
    let search_box = TextBox::new()
        .with_text_size(TEXT_SIZE_LARGE)
        .with_placeholder("Package name")
        .lens(AppState::search_term);

    let container = Container::new(search_box);
    let search_bar =
        Container::new(container.rounded(2.0).expand_width().padding(10.0)).expand_width();

    let list = Scroll::new(LensWrap::new(
        List::new(|| super::package_widget::package_widget()),
        AppState::package_list,
    ))
    .vertical();

    let no_packages_found_text = Either::new(
        |data: &AppState, _| data.no_packages_found,
        Label::dynamic(|data: &AppState, _| {
            format!("No packages found containing '{}'", data.last_search_term)
        })
        .with_text_size(TEXT_SIZE_NORMAL)
        .with_line_break_mode(druid::widget::LineBreaking::WordWrap),
        Flex::column(),
    );

    let search_buttons = Flex::row()
        .with_child(
            Button::dynamic(|data: &AppState, _| {
                if data.is_searching {
                    "Searching..."
                } else {
                    "Search Package"
                }
                .into()
            })
            .on_click(|ctx, data: &mut AppState, _| {
                super::controller::find_packages_async(data, ctx, false)
            })
            .disabled_if(|data: &AppState, _| data.is_searching),
        )
        .with_spacer(5.0)
        .with_child(Either::new(
            |data: &AppState, _| !data.is_searching,
            Button::new("Deep Search Package").on_click(|ctx, data: &mut AppState, _| {
                super::controller::find_packages_async(data, ctx, true)
            }),
            Flex::column(),
        ));

    let show_installed_packages_button = Button::new("Show installed packages").on_click(
        |ctx: &mut EventCtx, data: &mut AppState, _| {
            ctx.submit_command(Command::new(
                controller::FINISHED_SEARCH,
                data.installed_packages
                    .clone()
                    .into_iter()
                    .map(|cur_package| InstalledPackageWrapper {
                        visual_package_state: InstalledPackageState::Remove,
                        package: cur_package,
                    })
                    .collect(),
                Target::Global,
            ))
        },
    );

    let install_button = Button::new("Install added packages")
        .on_click(|ctx: &mut EventCtx, data: &mut AppState, _| {
            controller::install_packages(data, ctx);
        })
        .fix_size(250.0, 40.0);

    Flex::column()
        .with_flex_child(
            Flex::column()
                .with_child(search_bar)
                .with_child(search_buttons)
                .with_child(show_installed_packages_button)
                .with_child(no_packages_found_text)
                .with_flex_child(list, 1.0),
            1.0,
        )
        .with_spacer(6.0)
        .with_child(
            install_button
                .align_vertical(UnitPoint::BOTTOM)
                .align_right(),
        )
}
