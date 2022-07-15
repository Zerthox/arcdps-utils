//! Window component.

mod menu;
mod options;
mod render;

#[cfg(feature = "settings")]
mod settings;

use crate::ui::{
    render::{small_padding, window_context_menu},
    Component, Hideable, Ui, Windowable,
};
use std::ops::{Deref, DerefMut};

pub use menu::*;
pub use options::*;
pub use render::*;

#[cfg(feature = "settings")]
pub use settings::*;

/// Window component.
#[derive(Debug, Clone)]
pub struct Window<T> {
    pub options: WindowOptions,
    pub inner: T,
}

impl<'p, T> Window<T>
where
    T: Windowable<'p>,
{
    /// Creates a new window with [`WindowOptions`] and a given inner [`Windowable`] component.
    pub fn new(options: WindowOptions, inner: T) -> Self {
        Self { inner, options }
    }
}

impl<'p, T> Deref for Window<T>
where
    T: Windowable<'p>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'p, T> DerefMut for Window<T>
where
    T: Windowable<'p>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'p, T> Component<'p> for Window<T>
where
    T: Windowable<'p>,
{
    type Props = T::Props;

    fn render(&mut self, ui: &Ui, props: &'p Self::Props) {
        if let Some(_window) = render_window(ui, &mut self.options) {
            // update options
            self.options.update(ui);

            if T::CONTEXT_MENU {
                let pos = ui.window_pos();

                // render context menu
                window_context_menu(&format!("Options##{}", self.options.name), || {
                    let _style = small_padding(ui);

                    self.inner.render_menu(ui, props);
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

impl<'p, T> Hideable for Window<T>
where
    T: Windowable<'p>,
{
    fn is_visible(&self) -> bool {
        self.options.visible
    }

    fn visible_mut(&mut self) -> &mut bool {
        &mut self.options.visible
    }
}
