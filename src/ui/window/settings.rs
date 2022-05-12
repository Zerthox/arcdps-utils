use super::*;
use crate::settings::HasSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowSettings<T>
where
    T: HasSettings,
{
    shown: Option<bool>,
    settings: Option<T::Settings>,
}

impl<T> Default for WindowSettings<T>
where
    T: HasSettings,
    T::Settings: Default,
{
    fn default() -> Self {
        Self {
            shown: Some(true),
            settings: Some(T::Settings::default()),
        }
    }
}

impl<T> HasSettings for Window<T>
where
    T: Windowable + HasSettings,
{
    type Settings = WindowSettings<T>;

    const SETTINGS_ID: &'static str = T::SETTINGS_ID;

    fn current_settings(&self) -> Self::Settings {
        WindowSettings {
            shown: Some(self.is_visible()),
            settings: Some(self.inner.current_settings()),
        }
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        if let Some(shown) = loaded.shown {
            self.set_visibility(shown);
        }

        if let Some(settings) = loaded.settings {
            self.inner.load_settings(settings);
        }
    }
}
