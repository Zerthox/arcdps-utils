//! Load/save settings.

use arcdps::exports;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

/// Settings state.
#[derive(Debug, Clone)]
pub struct Settings {
    file: PathBuf,
    data: Map<String, Value>,
}

impl Settings {
    /// Creates new empty settings.
    pub fn empty(file: impl Into<PathBuf>) -> Self {
        Self {
            file: file.into(),
            data: Map::new(),
        }
    }

    /// Creates new settings from a file.
    pub fn from_file(file: impl AsRef<Path>) -> Self {
        let mut settings = Settings::empty(file.as_ref());
        settings.load_file();
        settings
    }

    /// Loads the settings from the settings file.
    ///
    /// This will return `false` if something fails.
    pub fn load_file(&mut self) -> bool {
        match self.load_file_helper() {
            Some(data) => {
                self.data = data;
                true
            }
            None => false,
        }
    }

    /// Helper to load data from the settings file.
    fn load_file_helper(&self) -> Option<Map<String, Value>> {
        let path = Self::config_path(&self.file)?;
        let reader = File::open(path).ok()?;
        serde_json::from_reader(BufReader::new(reader)).ok()
    }

    /// Saves settings to the settings file.
    ///
    /// This may silently fail.
    pub fn save_file(&self) {
        if let Some(path) = Self::config_path(&self.file) {
            if let Ok(file) = File::create(path) {
                #[allow(unused_must_use)]
                {
                    serde_json::to_writer_pretty(BufWriter::new(file), &self.data);
                }
            }
        }
    }

    /// Returns the path to the config file.
    pub fn config_path(file: impl AsRef<Path>) -> Option<PathBuf> {
        exports::config_path().map(|mut path| {
            if !path.is_dir() {
                path.pop();
            }
            path.push(file);
            path
        })
    }

    /// Loads data from the settings map.
    pub fn load_data<T>(&mut self, id: impl AsRef<str>) -> Option<T>
    where
        T: DeserializeOwned,
    {
        self.data
            .remove(id.as_ref())
            .and_then(|value| serde_json::from_value(value).ok())
    }

    /// Stores data in the settings map.
    ///
    /// Silently fails if the data fails serialization.
    pub fn store_data(&mut self, id: impl Into<String>, data: impl Serialize) {
        if let Ok(value) = serde_json::to_value(data) {
            self.data.insert(id.into(), value);
        }
    }

    /// Loads a component's settings from the settings map.
    pub fn load_component<T>(&mut self, component: &mut T)
    where
        T: HasSettings,
    {
        if let Some(loaded) = self.load_data(T::SETTINGS_ID) {
            component.load_settings(loaded);
        }
    }

    /// Stores a component's settings in the settings map.
    ///
    /// Silently fails if the component's settings fail serialization.
    pub fn store_component<T>(&mut self, component: &T)
    where
        T: HasSettings,
    {
        self.store_data(T::SETTINGS_ID, component.current_settings());
    }
}

/// Interface for components with settings.
pub trait HasSettings {
    type Settings: Serialize + DeserializeOwned;

    /// The component's settings id.
    const SETTINGS_ID: &'static str;

    /// Returns the component's current settings state.
    fn current_settings(&self) -> Self::Settings;

    /// Loads the component's settings from a loaded state.
    fn load_settings(&mut self, loaded: Self::Settings);

    /// Resets the component's settings to the defaults.
    fn reset_settings(&mut self)
    where
        Self::Settings: Default,
    {
        self.load_settings(Self::Settings::default())
    }
}

/// Helper for loading an optional value.
pub fn load_optional<T>(current: &mut T, loaded: Option<T>) {
    if let Some(value) = loaded {
        *current = value;
    }
}
