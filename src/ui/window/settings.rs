use super::*;
use crate::settings::{load_optional, HasSettings};
use serde::{Deserialize, Serialize};

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
    pub title_bar_background: Option<bool>,
    pub resize: Option<bool>,
    pub auto_resize: Option<bool>,
    pub scroll: Option<bool>,
    pub scroll_bar: Option<bool>,
    pub hotkey: Option<WindowHotkey>,

    pub settings: Option<T::Settings>,
}

impl<T> WindowSettings<T>
where
    T: HasSettings,
{
    pub fn new(options: &WindowOptions, settings: T::Settings) -> Self {
        Self {
            shown: Some(options.visible),
            position: Some(options.position.clone()),
            width: Some(options.width),
            height: Some(options.height),
            title_bar: Some(options.title_bar),
            background: Some(options.background),
            title_bar_background: Some(options.title_bar_background),
            resize: Some(options.resize),
            auto_resize: Some(options.auto_resize),
            scroll: Some(options.scroll),
            scroll_bar: Some(options.scroll_bar),
            hotkey: Some(options.hotkey),
            settings: Some(settings),
        }
    }
}

impl<T> Default for WindowSettings<T>
where
    T: HasSettings,
    T::Settings: Default,
{
    fn default() -> Self {
        Self::new(&WindowOptions::default(), T::Settings::default())
    }
}

impl<T> HasSettings for Window<T>
where
    T: HasSettings,
{
    type Settings = WindowSettings<T>;

    const SETTINGS_ID: &'static str = T::SETTINGS_ID;

    fn current_settings(&self) -> Self::Settings {
        WindowSettings::new(&self.options, self.inner.current_settings())
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        let Self::Settings {
            shown,
            position,
            width,
            height,
            title_bar,
            background,
            title_bar_background,
            resize,
            auto_resize,
            scroll,
            scroll_bar,
            hotkey,
            settings,
        } = loaded;

        load_optional(&mut self.options.visible, shown);
        load_optional(&mut self.options.position, position);
        load_optional(&mut self.options.width, width);
        load_optional(&mut self.options.height, height);
        load_optional(&mut self.options.title_bar, title_bar);
        load_optional(&mut self.options.background, background);
        load_optional(&mut self.options.title_bar_background, title_bar_background);
        load_optional(&mut self.options.resize, resize);
        load_optional(&mut self.options.auto_resize, auto_resize);
        load_optional(&mut self.options.scroll, scroll);
        load_optional(&mut self.options.scroll_bar, scroll_bar);
        load_optional(&mut self.options.hotkey, hotkey);

        if let Some(settings) = settings {
            self.inner.load_settings(settings);
        }
    }
}
