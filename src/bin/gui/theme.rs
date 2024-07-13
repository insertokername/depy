use druid::Env;

pub use druid::theme::*;

pub fn setup_theme<T>(env: &mut Env, _: &T) {
    env.set(TEXT_SIZE_NORMAL, 16.0);
}
