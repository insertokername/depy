use druid::{Color, Env};

pub use druid::theme::*;

/// Catpuccin Macchiato
#[allow(dead_code)]
fn setup_light(env: &mut Env){
    env.set(TEXT_COLOR,                 Color::rgb8(202, 211, 245));        // Text
    env.set(DISABLED_TEXT_COLOR,        Color::rgb8(128, 135, 162));        // Overlay1


    env.set(BUTTON_LIGHT,               Color::rgb8(24, 25, 38));           // Crust
    env.set(BUTTON_DARK,                Color::rgb8(24, 25, 38));           // Crust
    env.set(DISABLED_BUTTON_DARK,       Color::rgb8(36, 39, 58));           // Base
    env.set(DISABLED_BUTTON_LIGHT,      Color::rgb8(36, 39, 58));           // Base
    

    env.set(WINDOW_BACKGROUND_COLOR,    Color::rgb8(30, 32, 48));           // Mantle


    env.set(BORDER_DARK,                Color::rgb8(110, 115, 141));        // Overlay0
    env.set(BORDER_LIGHT,               Color::rgb8(165, 173, 203));        // Subtext0


    env.set(BACKGROUND_LIGHT,           Color::rgb8(30, 30, 46));           // Surface 0
    env.set(CURSOR_COLOR,               Color::rgb8(244, 219, 214));        // Rosewater
    env.set(PRIMARY_LIGHT,              Color::rgb8(147, 154, 183));        // Overlay 2

    env.set(PLACEHOLDER_COLOR,          Color::rgb8(91, 96, 120));          // Surface 2
}


/// Catpuccin Mocha
#[allow(dead_code)]
fn setup_dark(env: &mut Env){
    env.set(TEXT_COLOR,                 Color::rgb8(205, 214, 244));        // Text
    env.set(DISABLED_TEXT_COLOR,        Color::rgb8(127, 132, 156));        // Overlay1


    env.set(BUTTON_LIGHT,               Color::rgb8(17, 17, 27));           // Crust
    env.set(BUTTON_DARK,                Color::rgb8(17, 17, 27));           // Crust
    env.set(DISABLED_BUTTON_DARK,       Color::rgb8(30, 30, 46));           // Base
    env.set(DISABLED_BUTTON_LIGHT,      Color::rgb8(30, 30, 46));           // Base
    

    env.set(WINDOW_BACKGROUND_COLOR,    Color::rgb8(24, 24, 37));           // Mantle


    env.set(BORDER_DARK,                Color::rgb8(108, 112, 134));        // Overlay0
    env.set(BORDER_LIGHT,               Color::rgb8(166, 173, 200));        // Subtext0


    env.set(BACKGROUND_LIGHT,           Color::rgb8(30, 30, 46));           // Base
    env.set(CURSOR_COLOR,               Color::rgb8(245, 224, 220));        // Rosewater
    env.set(PRIMARY_LIGHT,              Color::rgb8(147, 153, 178));        // Overlay 2

    env.set(PLACEHOLDER_COLOR,          Color::rgb8(88, 91, 112));          // Surface 2
}

pub fn setup_theme<T>(env: &mut Env, _: &T) {
    env.set(TEXT_SIZE_NORMAL, 16.0);
    setup_light(env);
}
