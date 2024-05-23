use druid::widget::{Align, Flex, Label, TextBox, Button};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct AppState{
    value: i32,
    name: String,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = AppState {
        value: 30,
        name: "World".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &AppState, _env: &Env| format!("Hello {}!", data.name));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AppState::name);

    let btn_label = Label::dynamic(|data: &AppState, _env: &Env| format!("{}", data.value));
    let inc_button =
        Button::new("Increment").on_click(|_ctx, data: &mut AppState, _env| data.value += 1);
    let dec_button =
        Button::new("Decrement").on_click(|_ctx, data: &mut AppState, _env| data.value -= 1);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .with_child(btn_label)
        .with_spacer(8.0)
        .with_child(inc_button)
        .with_child(dec_button);

    // center the two widgets in the available space
    Align::centered(layout)

        
}