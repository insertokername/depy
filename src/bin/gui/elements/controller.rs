use std::{panic::catch_unwind, thread};

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
    #[error("Error: Thread paniced while cleaning up depy packages! Paniced on error {0}")]
    CleanupError(String),
    #[error("Error: Thread paniced while uninstalling depy! Paniced on error {0}")]
    UninstallErrror(String),
}

use crate::gui::app_state::{AppState, InstalledPackageState, InstalledPackageWrapper};

pub const FINISHED_SEARCH: Selector<Vector<InstalledPackageWrapper>> =
    Selector::new("finished-search");
const FAILED_SEARCH: Selector<String> = Selector::new("failed-search");

pub(super) const ADD_PACKAGE: Selector<package::Package> = Selector::new("add-package");
pub(super) const REMOVE_PACKAGE: Selector<package::Package> = Selector::new("remove-package");

const FINISHED_INSTALL: Selector<()> = Selector::new("finished-pacakges");
const FAILED_INSTALL: Selector<String> = Selector::new("failed-pacakges");

pub(super) const ADD_BUCKET: Selector<()> = Selector::new("add-bucket");
pub(super) const REMOVE_BUCKET: Selector<()> = Selector::new("remove-bucket");
pub(super) const UPDATE_BUCKETS: Selector<()> = Selector::new("update-buckets");

pub(super) const INITIALIZE: Selector<()> = Selector::new("initialize");
pub(super) const FINISHED_INITIALIZE: Selector<()> = Selector::new("finished_initialize");

pub(super) const CLEAN_DEPY: Selector<()> = Selector::new("clean-depy");
pub(super) const UNINSTALL_DEPY: Selector<()> = Selector::new("uninstall-depy");
pub(super) const FINISHED_CLEAN: Selector<()> = Selector::new("finished-clean");

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
            }
            if let Some(pkg) = cmd.get(ADD_PACKAGE) {
                if !data.installed_packages.contains(pkg) {
                    data.installed_packages.push_back(pkg.clone());
                    if let Some(changed_package) = data
                        .package_list
                        .iter_mut()
                        .find(|cur_package| cur_package.package.eq(pkg))
                    {
                        changed_package.visual_package_state = InstalledPackageState::Remove;
                    }
                }
            }
            if let Some(pkg) = cmd.get(REMOVE_PACKAGE) {
                data.installed_packages
                    .retain(|cur_package| cur_package.ne(pkg));
                if let Some(changed_package) = data
                    .package_list
                    .iter_mut()
                    .find(|cur_package| cur_package.package.eq(pkg))
                {
                    changed_package.visual_package_state = InstalledPackageState::AddPackage;
                }
                ctx.set_handled()
            }
            if let Some(()) = cmd.get(FINISHED_INSTALL) {}
            if let Some(err_msg) = cmd.get(FAILED_INSTALL) {
                match &mut data.error_message {
                    Some(some) => {
                        some.push_str("Got an error while installing: ");
                        some.push_str(&err_msg);
                        some.push('\n');
                    }
                    None => data.error_message = Some(err_msg.to_string()),
                }
            }
            if let Some(()) = cmd.get(ADD_BUCKET) {
                log::info!("Attempting to add bucket...");
                add_bucket(data, ctx);
            }
            if let Some(()) = cmd.get(REMOVE_BUCKET) {
                log::info!("Attempting to remove bucket...");
                remove_bucket(data, ctx);
            }
            if let Some(()) = cmd.get(UPDATE_BUCKETS) {
                data.bucket_list = bucket::list_buckets().unwrap().into();
            }
            if let Some(()) = cmd.get(INITIALIZE) {
                if std::path::Path::new("./depy.json").exists() {
                    data.installed_packages = Vector::from(
                        Package::multiple_packages_from_json(
                            &parse_json::read_json_file("./depy.json").unwrap(),
                        )
                        .unwrap(),
                    );
                } else {
                    log::info!("Couldn't find a depy.json file in the current directory!\n Depy will create a new file when installing the packages!")
                }
                data.initializing_depy = true;
                init_depy_async(ctx);
            }
            if let Some(()) = cmd.get(FINISHED_INITIALIZE) {
                data.initializing_depy = false;
            }
            if let Some(()) = cmd.get(CLEAN_DEPY) {
                data.is_cleaning_depy = true;
                clean_depy_async(ctx);
            }
            if let Some(()) = cmd.get(UNINSTALL_DEPY) {
                data.is_uninstalled = true;
                uninstall_depy_async();
            }
            if let Some(()) = cmd.get(FINISHED_CLEAN) {
                data.is_cleaning_depy = false;
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
        .collect::<Vec<package::Package>>();

    package::Package::save_packages_to_json(&package_vec).unwrap();

    let sink = ctx.get_external_handle();
    thread::spawn(move || {
        let result = catch_unwind(|| installer::install(package_vec));
        let flat_result = flatten_err(result, |panic_message| {
            return Box::new(ControllerError::InstallError(format!("{}", panic_message)));
        });

        match flat_result {
            Ok(()) => sink.submit_command(FINISHED_INSTALL, (), Target::Global),
            Err(err) => sink.submit_command(FAILED_INSTALL, err.to_string(), Target::Global),
        }
    });
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
                let _ = sink.submit_command(UPDATE_BUCKETS, (), Target::Global);
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
                let _ = sink.submit_command(UPDATE_BUCKETS, (), Target::Global);
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
            Box::new(ControllerError::ThreadSearchError(panic_message))
        });

        match flat_result {
            Ok(ok) => {
                let wrapped_ok: Vector<InstalledPackageWrapper> = ok
                    .into_iter()
                    .map(|pkg: package::Package| InstalledPackageWrapper {
                        visual_package_state: if installed_packages.contains(&pkg) {
                            InstalledPackageState::Remove
                        } else {
                            InstalledPackageState::AddPackage
                        },
                        package: pkg,
                    })
                    .collect();
                sink.submit_command(FINISHED_SEARCH, wrapped_ok, Target::Global)
            }
            Err(err) => sink.submit_command(FAILED_SEARCH, err.to_string(), Target::Global),
        }
    });
}

fn init_depy_async(ctx: &mut EventCtx) {
    let sink = ctx.get_external_handle();
    thread::spawn(move || {
        let result = catch_unwind(|| shell::init_depy());
        let flat_result = flatten_err(result, |panic_message| {
            Box::new(ControllerError::InitDepyError(panic_message))
        });

        match flat_result {
            Ok(()) => {
                log::info!("init successful!");
                let _ = sink.submit_command(FINISHED_INITIALIZE, (), Target::Global);
            }
            Err(err) => log::error!("Couldn't initialize depy! Got an error: {}", err),
        };
    });
}

fn uninstall_depy_async() {
    thread::spawn(move || {
        let result = catch_unwind(|| shell::uninstall_depy());
        let flat_result = flatten_err(result, |panic_message| {
            Box::new(ControllerError::UninstallErrror(panic_message))
        });
        match flat_result {
            Ok(()) => {
                log::info!("Uninstall successfull!");
                log::info!(
                    "Please uninstall depy from scoop as well, by using 'scoop uninstall depy'!"
                );
                log::info!("If you restart depy now, it will reinitialize automatically so it will leave junk in %userprofile%/depy!");
            }
            Err(err) => log::error!("Couldn't uninstall Depy! Got an error: {}", err),
        }
    });
}

fn clean_depy_async(ctx: &mut EventCtx) {
    let sink = ctx.get_external_handle();
    thread::spawn(move || {
        let result = catch_unwind(|| shell::clean_depy_packages());
        let flat_result = flatten_err(result, |panic_message| {
            Box::new(ControllerError::CleanupError(panic_message))
        });

        match flat_result {
            Ok(()) => {
                log::info!("Cleanup successfull!");
            }
            Err(err) => log::error!("Couldn't cleanup Depy! Got an error: {}", err),
        }
        let _ = sink.submit_command(FINISHED_CLEAN, (), Target::Global);
    });
}
