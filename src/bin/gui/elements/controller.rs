use std::{panic::catch_unwind, string, thread};

use depy::{
    bucket, installer,
    package::{self, Package},
    parse_json, shell,
};
use druid::{im::Vector, widget::Controller, Env, Event, EventCtx, Selector, Target, Widget};

#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("Error: Thread paniced while adding a bucket! Paniced on error {0}")]
    BucketAddError(String),
    #[error("Error: Thread paniced while removing a bucket! Paniced on error {0}")]
    BucketRemoveError(String),
    #[error("Error: Thread paniced while searching for a packages! Paniced on error {0}")]
    ThreadSearchError(String),
    #[error("Error: Thread paniced while installing packages! Paniced on error {0}")]
    InstallError(String),
    #[error("Error: Thread paniced while initializing depy! Paniced on error {0}")]
    InitDepyError(String),
}

use crate::gui::app_state::{AppState, InstalledPackageWrapper};

pub const FINISHED_SEARCH: Selector<Vector<InstalledPackageWrapper>> =
    Selector::new("finished-search");
const FAILED_SEARCH: Selector<String> = Selector::new("failed-search");
pub const UPDATE_PACKAGE_INSTALL_STATUS: Selector<package::Package> =
    Selector::new("update-package-install-status");

pub const FINISHED_INSTALL: Selector<()> = Selector::new("finished-pacakges");
pub const FAILED_INSTALL: Selector<String> = Selector::new("failed-pacakges");

pub const ADD_BUCKET: Selector<()> = Selector::new("add-bucket");
pub const REMOVE_BUCKET: Selector<()> = Selector::new("remove-bucket");
pub const UPDATE_BUCKETS: Selector<()> = Selector::new("update-buckets");

pub const INITIALIZE: Selector<()> = Selector::new("initialize");
pub const FINISHED_INITIALIZE: Selector<()> = Selector::new("finished_initialize");

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
            if let Some(err_msg) = cmd.get(FAILED_SEARCH) {
                data.package_list = druid::im::vector![];
                data.no_packages_found = false;
                data.last_search_term = data.search_term.clone();
                data.is_searching = false;
                match &mut data.error_message {
                    Some(some) => {
                        some.push_str(err_msg);
                        some.push('\n');
                    }
                    None => data.error_message = Some(err_msg.to_string()),
                }
                ctx.set_handled();
            }
            if let Some(pkg) = cmd.get(UPDATE_PACKAGE_INSTALL_STATUS) {
                if !data
                    .installed_packages
                    .iter()
                    .any(|cur_package| cur_package.equal(pkg))
                {
                    data.installed_packages.push_back(pkg.clone());
                    if let Some(changed_package) = data
                        .package_list
                        .iter_mut()
                        .find(|cur_package| cur_package.package.equal(pkg))
                    {
                        changed_package.is_installed = true;
                    }
                } else {
                    data.installed_packages
                        .retain(|cur_package| !cur_package.equal(pkg));
                    if let Some(changed_package) = data
                        .package_list
                        .iter_mut()
                        .find(|cur_package| cur_package.package.equal(pkg))
                    {
                        changed_package.is_installed = false;
                    }
                }

                ctx.set_handled();
            }
            if let Some(()) = cmd.get(FINISHED_INSTALL) {
                ctx.set_handled();
            }
            if let Some(err_msg) = cmd.get(FAILED_INSTALL) {
                match &mut data.error_message {
                    Some(some) => {
                        some.push_str("Got an error while installing: ");
                        some.push_str(&err_msg);
                        some.push('\n');
                    }
                    None => data.error_message = Some(err_msg.to_string()),
                }
                ctx.set_handled();
            }
            if let Some(()) = cmd.get(ADD_BUCKET) {
                log::info!("Attempting to add bucket...");
                add_bucket(data, ctx);
                ctx.set_handled();
            }
            if let Some(()) = cmd.get(REMOVE_BUCKET) {
                log::info!("Attempting to remove bucket...");
                remove_bucket(data, ctx);
                ctx.set_handled();
            }
            if let Some(()) = cmd.get(UPDATE_BUCKETS) {
                data.bucket_list = bucket::list_buckets().unwrap().into();
                ctx.set_handled();
            }
            if let Some(()) = cmd.get(INITIALIZE) {
                data.installed_packages = Vector::from(
                    Package::multiple_packages_from_json(
                        &parse_json::read_json_file("./depy.json").unwrap(),
                    )
                    .unwrap(),
                );
                data.initializing_depy = true;
                init_depy_gui(ctx);
                ctx.set_handled();
            }
            if let Some(()) = cmd.get(FINISHED_INITIALIZE) {
                data.initializing_depy = false;
                ctx.set_handled();
            }
        }

        child.event(ctx, event, data, env);
    }
}

/// Intended to be used to flatten the output of `catch_unwind`.
/// The wrapped result will be proccesd and flattened down to a single Result.
/// Uses the closure to proccess the unwind error into a flat error, the argument of the closure being the error message of the unwind error transformed to a string
fn flatten_err<T, FlatErr>(
    unflat_result: Result<Result<T, FlatErr>, Box<dyn std::any::Any + Send>>,
    process_unwind_error: impl Fn(String) -> FlatErr,
) -> Result<T, FlatErr> {
    match unflat_result {
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
            Err(process_unwind_error(panic_message))
        }
    }
}

pub fn install_packages(data: &mut AppState, ctx: &mut EventCtx) {
    let package_vec = data
        .installed_packages
        .clone()
        .into_iter()
        .map(|pkg| {
            let mut out = Package::from(pkg);
            out.version = "latest".to_string();
            out
        })
        .collect::<Vec<package::Package>>();

    package::Package::save_packages_to_json(&package_vec).unwrap();

    let sink = ctx.get_external_handle();
    thread::spawn(move || {
        let result = catch_unwind(|| installer::install(&package_vec));
        let flat_result = flatten_err(result, |panic_message| {
            return Box::new(ControllerError::InstallError(format!("{}", panic_message)));
        });

        match flat_result {
            Ok(()) => sink.submit_command(FINISHED_INSTALL, (), Target::Global),
            Err(err) => sink.submit_command(FAILED_INSTALL, err.to_string(), Target::Global),
        }
    });

    ctx.set_handled();
}

fn remove_bucket(data: &mut AppState, ctx: &mut EventCtx) {
    let bucket_name = data.add_bucket_name_field.clone();

    let sink = ctx.get_external_handle();

    thread::spawn(move || {
        let result = catch_unwind(|| bucket::remove_bucket(&bucket_name));
        let flat_result = flatten_err(result, |panic_message| {
            Box::new(ControllerError::BucketRemoveError(format!(
                "{}",
                panic_message
            )))
        });
        match flat_result {
            Ok(_) => {
                log::info!("Removing bucket!");
                sink.submit_command(UPDATE_BUCKETS, (), Target::Global);
            }
            Err(err) => log::error!("Got an error while removing bucket! {err}"),
        };
    });
}

fn add_bucket(data: &mut AppState, ctx: &mut EventCtx) {
    let bucket_name = data.add_bucket_name_field.clone();
    let bucket_url = data.add_bucket_url_field.clone();

    let sink = ctx.get_external_handle();

    thread::spawn(move || {
        let result = catch_unwind(|| bucket::add_bucket(&bucket_url, &bucket_name));
        let flat_result = flatten_err(result, |panic_message| {
            Box::new(ControllerError::BucketAddError(format!(
                "{}",
                panic_message
            )))
        });
        match flat_result {
            Ok(_) => {
                log::info!("Added bucket!");
                sink.submit_command(UPDATE_BUCKETS, (), Target::Global);
            }
            Err(err) => log::error!("Got an error while adding bucket! {err}"),
        };
    });
}

pub fn find_packages_async(data: &mut AppState, ctx: &mut EventCtx, deep_search: bool) {
    data.is_searching = true;

    let sink = ctx.get_external_handle();
    let search_term = data.search_term.clone();
    let installed_packages = data.installed_packages.clone();
    thread::spawn(move || {
        let result = catch_unwind(|| bucket::query_local_buckets(&search_term, deep_search));
        let flat_result = flatten_err(result, |panic_message| {
            Box::new(ControllerError::ThreadSearchError(format!(
                "{}",
                panic_message
            )))
        });

        match flat_result {
            Ok(ok) => {
                let wrapped_ok: Vector<InstalledPackageWrapper> = ok
                    .into_iter()
                    .map(|pkg: package::Package| InstalledPackageWrapper {
                        is_installed: installed_packages
                            .iter()
                            .any(|cur_package| cur_package.equal(&pkg)),
                        package: pkg,
                    })
                    .collect();
                sink.submit_command(FINISHED_SEARCH, wrapped_ok, Target::Global)
            }
            Err(err) => sink.submit_command(FAILED_SEARCH, err.to_string(), Target::Global),
        }
    });
}

fn init_depy_gui(ctx: &mut EventCtx) {
    let sink = ctx.get_external_handle();
    thread::spawn(move || {
        let result = catch_unwind(|| shell::init_depy());
        let flat_result = flatten_err(result, |panic_message| {
            Box::new(ControllerError::InitDepyError(format!("{}", panic_message)))
        });

        match flat_result {
            Ok(()) => {
                log::info!("init successful!");
                sink.submit_command(FINISHED_INITIALIZE, (), Target::Global);
            }
            Err(err) => log::error!("Couldn't initialize depy! Got an error: {}", err),
        };
    });
}
