//! Window component.

mod anchor;
mod menu;
mod options;
mod position;
mod render;

#[cfg(feature = "settings")]
mod settings;

use crate::ui::{
    render::{small_padding, window_context_menu},
    Component, Hideable, Ui, Windowable,
};
use std::ops::{Deref, DerefMut};

pub use anchor::*;
pub use menu::*;
pub use options::*;
pub use position::*;

#[cfg(feature = "settings")]
pub use settings::*;

/// Window component.
#[derive(Debug, Clone)]
pub struct Window<T> {
    /// The name (title) of the window.
    pub name: String,

    /// Inner [`Component`] responsible for rendering content.
    pub inner: T,

    /// Current window options state.
    pub options: WindowOptions,
}

impl<T> Window<T> {
    /// Creates a new window with [`WindowOptions`] and the passed [`Windowable`] component.
    pub fn new<P>(name: impl Into<String>, inner: T, options: WindowOptions) -> Self
    where
        T: Windowable<P>,
    {
        Self {
            name: name.into(),
            inner,
            options,
        }
    }

    /// Creates a new window with [`WindowOptions`] and a default [`Windowable`] component.
    pub fn with_default<P>(name: impl Into<String>, options: WindowOptions) -> Self
    where
        T: Default + Windowable<P>,
    {
        Self::new(name, T::default(), options)
    }

    /// Handles a key press event.
    ///
    /// Toggles visibility and returns `true` if the key matches the hotkey.
    #[inline]
    pub fn key_press(&mut self, key: usize) -> bool {
        self.options.key_press(key)
    }
}

impl<T> Deref for Window<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Window<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T, P> Component<P> for Window<T>
where
    T: Windowable<P>,
{
    fn render(&mut self, ui: &Ui, mut props: P) {
        if let Some(_window) = self.options.render_window(ui, &self.name) {
            // update options
            self.options.update(ui);

            if T::CONTEXT_MENU {
                let pos = ui.window_pos();

                // render context menu
                window_context_menu(format!("Options##{}", self.name), || {
                    let _style = small_padding(ui);

                    self.inner.render_menu(ui, &mut props);
                    if T::DEFAULT_OPTIONS {
                        window_options_menus(ui, &mut self.options, pos);
                    }
                });
            }

            // render window content
            self.inner.render(ui, props);
        }
    }
}

impl<T> Hideable for Window<T> {
    fn is_visible(&self) -> bool {
        self.options.is_visible()
    }

    fn visible_mut(&mut self) -> &mut bool {
        self.options.visible_mut()
    }
}
