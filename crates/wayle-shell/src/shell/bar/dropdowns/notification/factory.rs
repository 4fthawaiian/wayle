use tracing::warn;

use crate::shell::{
    bar::dropdowns::{DropdownFactory, DropdownInstance},
    services::ShellServices,
};

pub(crate) struct Factory;

impl DropdownFactory for Factory {
    fn create(_services: &ShellServices) -> Option<DropdownInstance> {
        warn!(dropdown = "notification", "notification service disabled, skipping dropdown");
        None
    }
}
