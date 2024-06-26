use depy::package;
use druid::{im::Vector, Data, Lens};

use super::logger::LogBuffer;

#[derive(Clone, Data, Lens)]
pub struct InstalledPackageWrapper{
    pub is_installed: bool,
    pub package: package::Package
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub is_searching: bool,
    pub error_message: Option<String>,
    pub search_term: String,
    pub last_search_term: String,
    pub package_list: Vector<InstalledPackageWrapper>,
    pub no_packages_found: bool,
    pub installed_packages: Vector<package::Package>,
    pub console_buff : LogBuffer, 
    pub cur_window: WindowSection,
    pub add_bucket_name_field: String,
    pub add_bucket_url_field: String,
    pub bucket_list: Vector<(String, String)>,
}

#[derive(Data, PartialEq, Clone)]
pub enum WindowSection{
    PackageSearch,
    BucketManagement
}

impl AppState {
    pub fn default() -> AppState {
        AppState {
            is_searching: false,
            error_message: None,
            search_term: "".to_string(),
            last_search_term: "".to_string(),
            package_list: Vector::new(),
            no_packages_found: false,
            installed_packages: Vector::new(),
            console_buff: LogBuffer::new(),
            cur_window: WindowSection::PackageSearch,
            add_bucket_name_field: "".to_string(),
            add_bucket_url_field: "".to_string(),
            bucket_list: Vector::new(),
        }
    }
}
