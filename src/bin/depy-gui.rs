use depy::{package::Package, parse_json};
use druid::{im::Vector, AppLauncher, LocalizedString, WindowDesc};
use env_logger::Target;

mod gui;

const WINDOW_TITLE: LocalizedString<gui::app_state::AppState> = LocalizedString::new("Depy");

fn main() {
    let main_window = WindowDesc::new(gui::elements::root_widget::root_widget())
        .title(WINDOW_TITLE)
        .window_size((600.0, 500.0));

    let mut initial_state = gui::app_state::AppState::default();

    let pipe = Target::Pipe(Box::new(initial_state.console_buff.clone_arc()));

    env_logger::Builder::new()
        .filter_level(if depy::ARGS.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .format_target(false)
        .format_timestamp(None)
        .target(pipe)
        .init();

    initial_state.installed_packages = Vector::from(
        Package::multiple_packages_from_json(&parse_json::read_json_file("./depy.json").unwrap())
            .unwrap(),
    );

    AppLauncher::with_window(main_window)
        .configure_env(gui::theme::setup_theme)
        .launch(initial_state)
        .expect("Failed to launch application");
}
