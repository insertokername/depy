pub mod bucket_management;
pub mod garbage_clean;
pub mod package_search;

use druid::{widget::ViewSwitcher, Insets, Widget, WidgetExt};

use crate::gui::app_state::{AppState, WindowSection};

pub fn make_window_picker() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _| data.cur_window.clone(),
        |section: &WindowSection, _, _| match section {
            WindowSection::PackageSearch => Box::new(package_search::make_package_search()),
            WindowSection::BucketManagement => {
                Box::new(bucket_management::make_bucket_management())
            }
            WindowSection::GarbageClean => Box::new(garbage_clean::make_garbage_clean()),
        },
    )
    .padding(Insets::uniform_xy(5.0, 15.0))
}
