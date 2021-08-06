//! Storage implementation

use crate::configuration::settings::Settings;

const KEY: &'static str = "yew.heat_eq.settings";

impl Settings {
    pub fn remove_and_default() -> Self {
        log::trace!("Removing values from storage");
        match Settings::remove() {
            Ok(()) => Settings::default(),
            Err(_) => {
                log::error!("Could not remove value!");
                Settings::default()
            }
        }
    }

    pub fn restore_or_default() -> Self {
        log::trace!("Restoring values from storage");
        match Settings::restore() {
            Ok(settings) => settings,
            Err(_) => {
                log::warn!("There are no records, we set settings to default");
                Settings::default()
            }
        }
    }

    pub fn remove() -> anyhow::Result<()> {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        local_storage
            .remove_item(KEY)
            .map_err(|_| anyhow::anyhow!("Failed to remove"))?;
        Ok(())
    }

    pub fn restore() -> anyhow::Result<Self> {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let settings = ron::de::from_str(
            &local_storage
                .get_item(KEY)
                .map_err(|_| anyhow::anyhow!("Could not get {}", KEY))?
                .ok_or_else(|| anyhow::anyhow!("Could not get {}", KEY))?,
        )?;
        Ok(settings)
    }

    pub fn store(&self) -> anyhow::Result<()> {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        local_storage
            .set_item(KEY, &ron::ser::to_string(self)?)
            .map_err(|_| anyhow::anyhow!("Failed to store"))?;
        Ok(())
    }
}
