//! This is the gui entry point of the program
//! 
//! It launches a druid window that has an `AppController` that calls depy library functions
//! 
//! This is also responsable for calling `depy::cleanup_path` after the program has closed
use clap::Parser;
use druid::{widget::Label, AppLauncher, LocalizedString, WindowDesc};
use env_logger::Target;
use gui::app_state::AppState;

/// Arguments parsing
mod args;
/// Druid `Widgets` and `AppControllers`
mod gui;

fn main() {
    const WINDOW_TITLE: LocalizedString<gui::app_state::AppState> = LocalizedString::new("Depy");

    let args = args::ArgsGui::parse();
    let main_window = WindowDesc::new(gui::elements::root_widget::root_widget())
        .title(WINDOW_TITLE)
        .window_size((800.0, 750.0));

    let initial_state = gui::app_state::AppState::default();

    let pipe = Target::Pipe(Box::new(initial_state.console_buff.log_buffer.clone_arc()));

    env_logger::Builder::new()
        .filter_level(if args.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .format_target(false)
        .format_timestamp(None)
        .target(pipe)
        .init();

    AppLauncher::with_window(main_window)
        .configure_env(gui::theme::setup_theme)
        .launch(initial_state)
        .expect("Failed to launch application");

    println!("cleaning up...\nPlease don't close terminal...");

    if let Err(err) = depy::shell::cleanup::cleanup_path() {
        let temp = AppState::default();

        AppLauncher::with_window(
            WindowDesc::new(Label::<AppState>::new(format!(
                "Got an error while cleaning up after depy!\nError was: {}",
                err.to_string()
            )))
            .title("Error!"),
        )
        .launch(temp)
        .unwrap();
    }

    println!("finished!");
}
