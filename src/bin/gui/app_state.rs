use depy::package;
use druid::{im::Vector, Data, Lens};

use super::logger::LogBuffer;

#[derive(Data, Clone, PartialEq)]
pub enum InstalledPackageState {
    AddPackage,
    SelectVersion,
    Remove,
}

#[derive(Clone, Data, Lens)]
pub struct InstalledPackageWrapper {
    pub visual_package_state: InstalledPackageState,
    pub package: package::Package,
}

#[derive(Clone, Data, Lens)]
pub struct LogBufferState {
    pub log_buffer: LogBuffer,
    pub is_bottom: bool,
}

impl LogBufferState {
    pub fn new(log_buffer: LogBuffer) -> LogBufferState {
        LogBufferState {
            log_buffer: log_buffer,
            is_bottom: false,
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub is_searching: bool,
    pub initializing_depy: bool,
    pub is_cleaning_depy: bool,
    pub is_uninstalled: bool,
    pub search_term: String,
    pub last_search_term: String,
    pub package_list: Vector<InstalledPackageWrapper>,
    pub no_packages_found: bool,
    pub installed_packages: Vector<package::Package>,
    pub console_buff: LogBufferState,
    pub cur_window: WindowSection,
    pub add_bucket_name_field: String,
    pub add_bucket_url_field: String,
    pub bucket_list: Vector<(String, String)>,
}

#[derive(Data, PartialEq, Clone)]
pub enum WindowSection {
    PackageSearch,
    BucketManagement,
    GarbageClean,
}

impl AppState {
    pub fn default() -> AppState {
        AppState {
            is_searching: false,
            initializing_depy: false,
            is_cleaning_depy: false,
            is_uninstalled: false,
            search_term: "".to_string(),
            last_search_term: "".to_string(),
            package_list: Vector::new(),
            no_packages_found: false,
            installed_packages: Vector::new(),
            console_buff: LogBufferState::new(LogBuffer::new()),
            cur_window: WindowSection::PackageSearch,
            add_bucket_name_field: "".to_string(),
            add_bucket_url_field: "".to_string(),
            bucket_list: Vector::new(),
        }
    }
}
