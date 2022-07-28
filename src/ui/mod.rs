//! UI related utilities.

pub mod align;
pub mod render;
pub mod window;

#[cfg(feature = "log")]
pub mod log;

pub use arcdps::imgui::Ui;
pub use window::{Window, WindowOptions};

/// Interface for UI components.
pub trait Component<Props> {
    /// Renders the component.
    fn render(&mut self, ui: &Ui, props: Props);
}

impl<T, P> Component<P> for T
where
    T: FnMut(&Ui, P),
{
    fn render(&mut self, ui: &Ui, props: P) {
        self(ui, props)
    }
}

/// Interface for windowable UI components.
pub trait Windowable<Props>: Component<Props> {
    /// Whether to enable the context menu.
    const CONTEXT_MENU: bool;

    /// Whether to enable the default menu entries.
    const DEFAULT_OPTIONS: bool = true;

    /// Renders the window context menu contents.
    fn render_menu(&mut self, _ui: &Ui, _props: &Props) {}
}

/// Interface for hideable UI components.
pub trait Hideable {
    /// Returns whether the component is currently visible.
    fn is_visible(&self) -> bool;

    /// Returns a mutable reference to the component's visibility state.
    fn visible_mut(&mut self) -> &mut bool;

    /// Hides the component.
    fn hide(&mut self) {
        *self.visible_mut() = false;
    }

    /// Shows the component.
    fn show(&mut self) {
        *self.visible_mut() = true;
    }

    /// Toggles the component's visibility.
    fn toggle_visibility(&mut self) {
        let shown = self.visible_mut();
        *shown = !*shown;
    }

    /// Sets the component's visibility state.
    fn set_visibility(&mut self, visible: bool) {
        *self.visible_mut() = visible;
    }
}
