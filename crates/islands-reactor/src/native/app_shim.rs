use crate::bindings::Application;
use windows_core::Result;

pub(crate) fn create_island_application() -> Result<Application> {
    Application::new()
}
