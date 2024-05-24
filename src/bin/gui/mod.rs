use druid::{Data, Lens};

pub mod elements;
pub mod theme;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    value: i32,
    search_term: String,
}

impl AppState {
    pub fn new(value: i32, search_term: String) -> AppState {
        AppState { value, search_term }
    }
}
