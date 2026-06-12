use relm4::prelude::*;

use super::{NotificationDropdown, messages::NotificationDropdownInit};
use crate::shell::{
    bar::dropdowns::{DropdownFactory, DropdownInstance},
    services::ShellServices,
};

pub(crate) struct Factory;

impl DropdownFactory for Factory {
    fn create(services: &ShellServices) -> Option<DropdownInstance> {
        let notification_enabled = services.config.config().modules.notification.enabled.get();
        let notification = services.notification.clone()?;

        if !notification_enabled {
            return None;
        }

        let init = NotificationDropdownInit {
            notification,
            config: services.config.clone(),
        };
        let controller = NotificationDropdown::builder().launch(init).detach();

        let popover = controller.widget().clone();
        Some(DropdownInstance::new(popover, Box::new(controller)))
    }
}
