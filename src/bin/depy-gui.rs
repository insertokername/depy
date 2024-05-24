use druid::widget::{Container, Flex, Label, Scroll, TextBox};
use druid::{
    AppLauncher, Color, Data, Env, Lens, LocalizedString, UnitPoint, Widget, WidgetExt, WindowDesc,
};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Depy");

#[derive(Clone, Data, Lens)]
struct AppState {
    value: i32,
    search_term: String,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = AppState {
        value: 30,
        search_term: "".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    let search_box = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .lens(AppState::search_term);
    let search_bar = Container::new(
        Container::new(search_box)
            .rounded(2.0)
            .expand_width()
            .padding(10.0),
    )
    .expand_width()
    .border(Color::RED, 1.0);

    let list = {
        let mut col = Flex::column();
        let cols = 30;

        for _ in 0..cols {
            col.add_child(Label::new("20"));
        }

        Scroll::new(
            Container::new(col)
                .border(Color::RED, 1.0)
                .padding(10.0),
        )
        .expand_width()
    };

    Flex::column()
        .with_child(search_bar)
        .with_flex_child(list, 1.0)
}
