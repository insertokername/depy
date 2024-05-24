use druid::{AppLauncher, LocalizedString, WindowDesc};
use gui::{elements, theme, AppState};

mod gui;

const WINDOW_TITLE: LocalizedString<gui::AppState> = LocalizedString::new("Depy");

fn main() {
    let main_window = WindowDesc::new(elements::build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((600.0, 500.0));

    let initial_state = AppState::default();

    AppLauncher::with_window(main_window)
        .configure_env(theme::setup_theme)
        .launch(initial_state)
        .expect("Failed to launch application");
}
