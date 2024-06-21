use std::panic::catch_unwind;
use std::thread;

use depy::package::{self, Package};
use druid::im::{vector, Vector};
use druid::widget::{
    Button, Container, Controller, Either, Flex, Label, LensWrap, List, Scroll, TextBox,
};
use druid::{theme::*, Color, Env, Event, EventCtx, Selector, Target};
use druid::{Widget, WidgetExt};

use super::AppState;

const FINISHED_SEARCH: Selector<Vector<Package>> = Selector::new("finished-search");
const FAILED_SEARCH: Selector<String> = Selector::new("failed-search");

struct AppController;

impl<W: Widget<AppState>> Controller<AppState, W> for AppController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if let Event::Command(cmd) = event {
            if let Some(list) = cmd.get(FINISHED_SEARCH) {
                data.package_list = list.clone();
                data.no_packages_found = list.is_empty();
                data.last_search_term = data.search_term.clone();
                data.is_searching = false;
                ctx.set_handled();
            }
        }
        if let Event::Command(cmd) = event {
            if let Some(err_msg) = cmd.get(FAILED_SEARCH) {
                data.package_list = vector![];
                data.no_packages_found = false;
                data.last_search_term = data.search_term.clone();
                data.is_searching = false;
                match &mut data.error_message {
                    Some(some) => {
                        some.push('\n');
                        some.push_str(err_msg)
                    }
                    None => data.error_message = Some(err_msg.clone()),
                }
                ctx.set_handled();
            }
        }
        child.event(ctx, event, data, env);
    }
}

fn find_packages(
    data: &mut AppState,
    ctx: &mut EventCtx,
    deep_search: bool,
) {
    data.is_searching = true;

    let sink = ctx.get_external_handle();
    let search_term = data.search_term.clone();
    thread::spawn(move || {
        let result = catch_unwind(|| depy::bucket::query_local_buckets(&search_term, deep_search));
        let flat_result = match result {
            Ok(ok) => ok,
            Err(err) => {
                let panic_message = if let Some(message) = err.downcast_ref::<&str>() {
                    message.to_string()
                } else if let Some(message) = err.downcast_ref::<String>() {
                    message.clone()
                } else {
                    "Unknown panic occurred".to_string()
                };
                println!("Errored out on error: {:?}", panic_message);
                Result::<Vector<Package>, Box<dyn std::error::Error>>::Err(
                    depy::bucket::BucketError::ThreadSearchError(format!("{}", panic_message))
                        .into(),
                )
            }
        };

        match flat_result {
            Ok(ok) => sink.submit_command(FINISHED_SEARCH, ok, Target::Global),
            Err(err) => sink.submit_command(FAILED_SEARCH, err.to_string(), Target::Global),
        }
    });
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

    let error_box = Either::new(
        |data: &AppState, _| data.error_message.is_some(),
        Flex::column()
            .with_child(
                Button::new("Clean errors")
                    .on_click(|_, data: &mut AppState, _| data.error_message = None),
            )
            .with_flex_child(
                Label::dynamic(|data: &AppState, _| {
                    match &data.error_message {
                        Some(some) => some,
                        None => "Error while loading an error message!",
                    }
                    .to_string()
                })
                .with_text_size(TEXT_SIZE_LARGE)
                .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
                .with_text_color(Color::RED)
                .scroll()
                .vertical(),
                1.0,
            ),
        Flex::column(),
    );

    let no_packages_found_text = Either::new(
        |data: &AppState, _| data.no_packages_found,
        Label::dynamic(|data: &AppState, _| {
            format!("No packages found containing '{}'", data.last_search_term)
        })
        .with_text_size(TEXT_SIZE_NORMAL)
        .with_line_break_mode(druid::widget::LineBreaking::WordWrap),
        Flex::column(),
    );
    let other_list = Scroll::new(LensWrap::new(
        // List::new(|| Label::dynamic(|data, _| format!("List item: {data}"))),
        List::new(|| package_widget()),
        AppState::package_list,
    ))
    .expand_width();

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
            .on_click(|ctx, data: &mut AppState, _| find_packages(data, ctx, false)),
        )
        .with_spacer(5.0)
        .with_child(Either::new(
            |data: &AppState, _| data.no_packages_found && !data.is_searching,
            Button::new("Deep Search Package")
                .on_click(|ctx, data: &mut AppState, _| find_packages(data, ctx, true)),
            Flex::column(),
        ));

    Flex::column()
        .with_child(search_bar)
        .with_child(search_buttons)
        .with_child(no_packages_found_text)
        .with_flex_child(error_box, 1.0)
        .with_flex_child(other_list, 100.0)
        .controller(AppController)
}
