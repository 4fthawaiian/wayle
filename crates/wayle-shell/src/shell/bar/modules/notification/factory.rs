use std::rc::Rc;

use tracing::warn;
use wayle_widgets::prelude::BarSettings;

use crate::shell::{
    bar::{
        dropdowns::DropdownRegistry,
        modules::registry::{ModuleFactory, ModuleInstance},
    },
    services::ShellServices,
};

pub(crate) struct Factory;

impl ModuleFactory for Factory {
    fn create(
        _settings: &BarSettings,
        _services: &ShellServices,
        _dropdowns: &Rc<DropdownRegistry>,
        _class: Option<String>,
    ) -> Option<ModuleInstance> {
        warn!(module = "notification", "notification service disabled, skipping module");
        None
    }
}
