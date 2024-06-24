use std::{panic::catch_unwind, thread};

use depy::{bucket, package};
use druid::{
    im::{vector, Vector},
    widget::Controller,
    Env, Event, EventCtx, Selector, Target, Widget,
};

use crate::gui::app_state::AppState;

const FINISHED_SEARCH: Selector<Vector<package::Package>> = Selector::new("finished-search");
const FAILED_SEARCH: Selector<String> = Selector::new("failed-search");

pub struct AppController;

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
                    None => data.error_message = Some(err_msg.to_string()),
                }
                ctx.set_handled();
            }
        }
        child.event(ctx, event, data, env);
    }
}

pub fn find_packages_async(
    data: &mut AppState,
    ctx: &mut EventCtx,
    deep_search: bool,
) {
    data.is_searching = true;

    let sink = ctx.get_external_handle();
    let search_term = data.search_term.clone();
    thread::spawn(move || {
        let result = catch_unwind(|| bucket::query_local_buckets(&search_term, deep_search));
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
                Result::<Vector<package::Package>, Box<dyn std::error::Error>>::Err(
                    bucket::BucketError::ThreadSearchError(format!("{}", panic_message))
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
