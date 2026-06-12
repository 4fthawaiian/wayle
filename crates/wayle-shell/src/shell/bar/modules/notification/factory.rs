use std::rc::Rc;

use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use crate::shell::{
    bar::{
        dropdowns::DropdownRegistry,
        modules::registry::{ModuleFactory, ModuleInstance, dynamic_controller},
    },
    services::ShellServices,
};

use super::{Factory, NotificationInit, NotificationModule};

impl ModuleFactory for Factory {
    fn create(
        settings: &BarSettings,
        services: &ShellServices,
        dropdowns: &Rc<DropdownRegistry>,
        class: Option<String>,
    ) -> Option<ModuleInstance> {
        let notification_enabled = services.config.config().modules.notification.enabled.get();
        let notification_service = services.notification.as_ref()?;

        if !notification_enabled {
            return None;
        }

        let init = NotificationInit {
            settings: settings.clone(),
            notification: notification_service.clone(),
            config: services.config.clone(),
            dropdowns: dropdowns.clone(),
        };
        let controller = dynamic_controller(NotificationModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
