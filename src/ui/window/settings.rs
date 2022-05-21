use super::*;
use crate::settings::{load_optional, HasSettings};
use serde::{Deserialize, Serialize};

// TODO: use window options struct?
#[derive(Debug, Serialize, Deserialize)]
pub struct WindowSettings<T>
where
    T: HasSettings,
{
    pub shown: Option<bool>,
    pub position: Option<WindowPosition>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub title_bar: Option<bool>,
    pub background: Option<bool>,
    pub resize: Option<bool>,
    pub auto_resize: Option<bool>,
    pub scroll: Option<bool>,
    pub scroll_bar: Option<bool>,
    pub settings: Option<T::Settings>,
}

impl<T> Default for WindowSettings<T>
where
    T: HasSettings,
    T::Settings: Default,
{
    fn default() -> Self {
        Self {
            shown: Some(true),
            position: Some(WindowPosition::Manual),
            width: None,
            height: None,
            title_bar: Some(true),
            background: Some(true),
            resize: Some(true),
            auto_resize: Some(false),
            scroll: Some(true),
            scroll_bar: Some(true),
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
            shown: Some(self.options.visible),
            position: Some(self.options.position.clone()),
            width: Some(self.options.width),
            height: Some(self.options.height),
            title_bar: Some(self.options.title_bar),
            background: Some(self.options.background),
            resize: Some(self.options.resize),
            auto_resize: Some(self.options.auto_resize),
            scroll: Some(self.options.scroll),
            scroll_bar: Some(self.options.scroll_bar),
            settings: Some(self.inner.current_settings()),
        }
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        load_optional(&mut self.options.visible, loaded.shown);
        load_optional(&mut self.options.position, loaded.position);
        load_optional(&mut self.options.width, loaded.width);
        load_optional(&mut self.options.height, loaded.height);
        load_optional(&mut self.options.title_bar, loaded.title_bar);
        load_optional(&mut self.options.background, loaded.background);
        load_optional(&mut self.options.resize, loaded.resize);
        load_optional(&mut self.options.auto_resize, loaded.auto_resize);
        load_optional(&mut self.options.scroll, loaded.scroll);
        load_optional(&mut self.options.scroll_bar, loaded.scroll_bar);

        if let Some(settings) = loaded.settings {
            self.inner.load_settings(settings);
        }
    }
}
