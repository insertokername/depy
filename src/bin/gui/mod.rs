use depy::package;
use druid::{im::Vector, Data, Lens};

pub mod elements;
pub mod theme;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    search_term: String,
    package_list: Vector<package::Package>,
}

impl AppState {
    pub fn default() -> AppState {
        AppState {
            search_term: "".to_string(),
            package_list: Vector::new(),
        }
    }
}
