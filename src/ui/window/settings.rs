use super::*;
use crate::settings::HasSettings;
use serde::{Deserialize, Serialize};

// TODO: use window options struct?
#[derive(Debug, Serialize, Deserialize)]
pub struct WindowSettings<T>
where
    T: HasSettings,
{
    #[serde(flatten)]
    pub options: WindowOptions,
    pub settings: Option<T::Settings>,
}

impl<T> Default for WindowSettings<T>
where
    T: HasSettings,
    T::Settings: Default,
{
    fn default() -> Self {
        Self {
            options: WindowOptions::default(),
            settings: Some(T::Settings::default()),
        }
    }
}

impl<T> HasSettings for Window<T>
where
    T: HasSettings,
{
    type Settings = WindowSettings<T>;

    const SETTINGS_ID: &'static str = T::SETTINGS_ID;

    fn current_settings(&self) -> Self::Settings {
        WindowSettings {
            options: self.options.clone(),
            settings: Some(self.inner.current_settings()),
        }
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        self.options = loaded.options;
        if let Some(settings) = loaded.settings {
            self.inner.load_settings(settings);
        }
    }
}
