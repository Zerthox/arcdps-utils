//! Window component.

use crate::ui::{Component, Hideable};
use arcdps::imgui::{self, Ui};
use std::ops::{Deref, DerefMut};

/// Window component.
#[derive(Debug, Clone)]
pub struct Window<T>
where
    T: Component,
{
    pub inner: T,
    pub name: String,
    pub visible: bool,
    pub width: f32,
    pub height: f32,
    pub resize: bool,
    pub auto_resize: bool,
    pub scroll: bool,
}

impl<T> Window<T>
where
    T: Component,
{
    /// Creates a new window with a given inner [`Component`].
    pub fn new(name: impl Into<String>, inner: T) -> Self {
        Self {
            inner,
            name: name.into(),
            visible: true,
            width: 0.0,
            height: 0.0,
            resize: true,
            auto_resize: false,
            scroll: true,
        }
    }

    /// Sets the window name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets whether the window is visible.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Sets the default window width.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Sets the default window height.
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Sets whether the window is resizable.
    pub fn resize(mut self, value: bool) -> Self {
        self.resize = value;
        self
    }

    /// Sets whether the window automatically resizes.
    pub fn auto_resize(mut self, value: bool) -> Self {
        self.auto_resize = value;
        self
    }

    /// Sets whether the window is scrollable.
    pub fn scroll(mut self, value: bool) -> Self {
        self.scroll = value;
        self
    }
}

impl<T> Window<T>
where
    T: Component + Default,
{
    /// Creates a new window with the [`Default`] of the inner [`Component`].
    pub fn with_default(name: impl Into<String>) -> Self {
        Self::new(name, T::default())
    }
}

impl<T> Deref for Window<T>
where
    T: Component,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Window<T>
where
    T: Component,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Component for Window<T>
where
    T: Component,
{
    type Props = T::Props;

    fn render(&mut self, ui: &Ui, props: &Self::Props) {
        if self.visible {
            let inner = &mut self.inner;

            imgui::Window::new(&self.name)
                .title_bar(true)
                .draw_background(true)
                .collapsible(false)
                .size([self.width, self.height], imgui::Condition::FirstUseEver)
                .always_auto_resize(self.auto_resize)
                .resizable(self.resize)
                .scroll_bar(self.scroll)
                .scrollable(self.scroll)
                .focus_on_appearing(false)
                .opened(&mut self.visible)
                .build(ui, || inner.render(ui, props));
        }
    }
}

impl<T> Hideable for Window<T>
where
    T: Component,
{
    fn is_visible(&self) -> bool {
        self.visible
    }
    fn is_visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
}

#[cfg(feature = "settings")]
mod settings {
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
        T: Component + HasSettings,
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
            // self.set_hotkey(loaded.hotkey);
            if let Some(settings) = loaded.settings {
                self.inner.load_settings(settings);
            }
        }
    }
}

#[cfg(feature = "settings")]
pub use settings::*;
