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
    inner: T,
    shown: bool,
}

impl<T> Window<T>
where
    T: Component,
{
    /// Creates a new window with a given inner [`Component`].
    pub fn new(inner: T) -> Self {
        Self { inner, shown: true }
    }

    /// Returns a reference to the inner [`Component`].
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Returns a mutable reference to the inner [`Component`].
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> Default for Window<T>
where
    T: Component + Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Deref for Window<T>
where
    T: Component,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<T> DerefMut for Window<T>
where
    T: Component,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

#[derive(Debug, Clone)]
pub struct WindowProps {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub resize: bool,
    pub auto_resize: bool,
    pub scroll: bool,
}

impl WindowProps {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
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

    /// Sets the default window width.
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Sets the default window height.
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Sets whether the window is resizable.
    pub const fn resize(mut self, value: bool) -> Self {
        self.resize = value;
        self
    }

    /// Sets whether the window automatically resizes.
    pub const fn auto_resize(mut self, value: bool) -> Self {
        self.auto_resize = value;
        self
    }

    /// Sets whether the window is scrollable.
    pub const fn scroll(mut self, value: bool) -> Self {
        self.scroll = value;
        self
    }
}

impl<T> Component for Window<T>
where
    T: Component,
{
    type Props = (WindowProps, T::Props);

    fn render(&mut self, ui: &Ui, props: &Self::Props) {
        if self.shown {
            let (window_props, inner_props) = props;
            let inner = &mut self.inner;

            imgui::Window::new(&window_props.name)
                .title_bar(true)
                .draw_background(true)
                .collapsible(false)
                .size(
                    [window_props.width, window_props.height],
                    imgui::Condition::FirstUseEver,
                )
                .always_auto_resize(window_props.auto_resize)
                .resizable(window_props.resize)
                .scroll_bar(window_props.scroll)
                .scrollable(window_props.scroll)
                .focus_on_appearing(false)
                .opened(&mut self.shown)
                .build(ui, || inner.render(ui, inner_props));
        }
    }
}

impl<T> Hideable for Window<T>
where
    T: Component,
{
    fn is_visible(&self) -> bool {
        self.shown
    }
    fn is_visible_mut(&mut self) -> &mut bool {
        &mut self.shown
    }
}

#[cfg(feature = "settings")]
mod settings {
    use super::*;
    use crate::settings::HasSettings;
    use serde_crate::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(crate = "serde_crate")]
    pub struct WindowSettings<T>
    where
        T: HasSettings,
    {
        shown: Option<bool>,
        hotkey: Option<usize>,
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
                hotkey: None,
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
                hotkey: None,
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
