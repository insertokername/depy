use std::{panic::catch_unwind, thread};

use depy::{
    bucket, installer,
    package::{self, Package},
};
use druid::{im::Vector, widget::Controller, Env, Event, EventCtx, Selector, Target, Widget};

#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("Error: Thread paniced while searching for a packages! Paniced on error {0}")]
    ThreadSearchError(String),
    #[error("Error: Thread paniced while installing packages! Paniced on error {0}")]
    InstallError(String),
}

use crate::gui::app_state::{AppState, InstalledPackageWrapper};

pub const FINISHED_SEARCH: Selector<Vector<InstalledPackageWrapper>> =
    Selector::new("finished-search");
const FAILED_SEARCH: Selector<String> = Selector::new("failed-search");
pub const UPDATE_PACKAGE_INSTALL_STATUS: Selector<package::Package> =
    Selector::new("update-package-install-status");
pub const FINISHED_INSTALL: Selector<()> = Selector::new("finished-pacakges");
pub const FAILED_INSTALL: Selector<String> = Selector::new("failed-pacakges");

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
                println!("finished")
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
            }
        }

        child.event(ctx, event, data, env);
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
                Result::<(), Box<dyn std::error::Error>>::Err(
                    ControllerError::InstallError(format!("{}", panic_message)).into(),
                )
            }
        };

        match flat_result {
            Ok(()) => sink.submit_command(FINISHED_INSTALL, (), Target::Global),
            Err(err) => sink.submit_command(FAILED_INSTALL, err.to_string(), Target::Global),
        }
    });

    ctx.set_handled();
}

pub fn find_packages_async(data: &mut AppState, ctx: &mut EventCtx, deep_search: bool) {
    data.is_searching = true;

    let sink = ctx.get_external_handle();
    let search_term = data.search_term.clone();
    let installed_packages = data.installed_packages.clone();
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
                    ControllerError::ThreadSearchError(format!("{}", panic_message)).into(),
                )
            }
        };

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
