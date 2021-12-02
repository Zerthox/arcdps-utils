//! Window component.

use crate::ui::{Component, Hideable, Windowed};
use arcdps::imgui::{self, Condition, ImString, Ui};
use std::ops::{Deref, DerefMut};

/// Window component.
#[derive(Debug)]
pub struct Window<T>
where
    T: Component,
{
    props: WindowProps,
    inner: T,
    shown: bool,
}

impl<T> Window<T>
where
    T: Component,
{
    /// Creates a new window with a given inner [`Component`].
    pub fn with_inner(props: WindowProps, inner: T) -> Self {
        let shown = props.visible;
        Self {
            props,
            inner,
            shown,
        }
    }

    /// Returns a reference to the inner [`Component`].
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Returns a mutable reference to the inner [`Component`].
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Returns the window hotkey.
    pub fn hotkey(&self) -> Option<usize> {
        self.props.hotkey
    }

    /// Sets the window hotkey.
    pub fn set_hotkey(&mut self, key: Option<usize>) {
        self.props.hotkey = key
    }
}

impl<T> Window<T>
where
    T: Component + Default,
{
    /// Creates a new window with the [`Default`] value of the inner [`Component`].
    pub fn with_default(props: WindowProps) -> Self {
        Self::with_inner(props, T::default())
    }
}

impl<T> Default for Window<T>
where
    T: Default + Windowed,
{
    fn default() -> Self {
        T::default().windowed()
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

impl<T> Component for Window<T>
where
    T: Component,
{
    fn render(&mut self, ui: &Ui) {
        if self.shown {
            let inner = &mut self.inner;
            self.props
                .new_window()
                .opened(&mut self.shown)
                .build(ui, || inner.render(ui))
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

/// Window props.
#[derive(Debug, Clone)]
pub struct WindowProps {
    name: ImString,
    width: f32,
    height: f32,
    visible: bool,
    resize: bool,
    auto_resize: bool,
    scroll: bool,
    hotkey: Option<usize>,
}

impl WindowProps {
    /// Creates a new set of window props.
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: ImString::new(name.into()),
            width: 0.0,
            height: 0.0,
            resize: true,
            visible: true,
            auto_resize: false,
            scroll: true,
            hotkey: None,
        }
    }

    /// Sets the window name.
    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.name = ImString::new(name.into());
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

    /// Sets whether the window is visible.
    pub const fn visible(mut self, value: bool) -> Self {
        self.visible = value;
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

    /// Sets the window hotkey
    pub const fn hotkey(mut self, value: usize) -> Self {
        self.hotkey = Some(value);
        self
    }

    /// Creates the [`imgui::Window`] corresponding to the props.
    fn new_window(&self) -> imgui::Window {
        imgui::Window::new(&self.name)
            .title_bar(true)
            .draw_background(true)
            .collapsible(false)
            .size([self.width, self.height], Condition::FirstUseEver)
            .always_auto_resize(self.auto_resize)
            .resizable(self.resize)
            .scroll_bar(self.scroll)
            .scrollable(self.scroll)
            .focus_on_appearing(false)
    }
}

#[cfg(feature = "settings")]
mod settings {
    use crate::{
        settings::HasSettings,
        ui::{Component, Hideable, Window, Windowed},
    };
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

    impl<T> HasSettings for Window<T>
    where
        T: Component + Windowed + HasSettings,
    {
        type Settings = WindowSettings<T>;

        fn settings_id() -> &'static str {
            T::settings_id()
        }

        fn get_settings(&self) -> Self::Settings {
            WindowSettings {
                shown: Some(self.is_visible()),
                hotkey: self.hotkey(),
                settings: Some(self.inner.get_settings()),
            }
        }

        fn load_settings(&mut self, loaded: Self::Settings) {
            if let Some(shown) = loaded.shown {
                self.set_visibility(shown);
            }
            self.set_hotkey(loaded.hotkey);
            if let Some(settings) = loaded.settings {
                self.inner.load_settings(settings);
            }
        }
    }
}

#[cfg(feature = "settings")]
pub use settings::*;
