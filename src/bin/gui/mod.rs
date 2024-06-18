use depy::package;
use druid::{im::Vector, Data, Lens};

pub mod elements;
pub mod theme;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    is_searching: bool,
    error_message: Option<String>,
    search_term: String,
    package_list: Vector<package::Package>,
}

impl AppState {
    pub fn default() -> AppState {
        AppState {
            is_searching: false,
            error_message: None,
            search_term: "".to_string(),
            package_list: Vector::new(),
        }
    }
}
