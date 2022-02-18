//! UI related utilities.

pub mod align;
pub mod components;

pub use arcdps::imgui::Ui;
pub use components::window::{Window, WindowProps};

/// Interface for UI components.
pub trait Component {
    /// Renders the component.
    fn render(&mut self, ui: &Ui);
}

/// Interface for hideable UI components.
pub trait Hideable
where
    Self: Component,
{
    /// Returns whether the component is currently visible.
    fn is_visible(&self) -> bool;

    /// Returns a mutable reference to the component's visibility state.
    fn is_visible_mut(&mut self) -> &mut bool;

    /// Hides the component.
    fn hide(&mut self) {
        *self.is_visible_mut() = false;
    }

    /// Shows the component.
    fn show(&mut self) {
        *self.is_visible_mut() = true;
    }

    /// Toggles the component's visibility.
    fn toggle_visibility(&mut self) {
        let shown = self.is_visible_mut();
        *shown = !*shown;
    }

    /// Sets the component's visibility state.
    fn set_visibility(&mut self, visible: bool) {
        *self.is_visible_mut() = visible;
    }
}

/// Interface for window UI components.
pub trait Windowed
where
    Self: Component + Sized,
{
    /// Returns the default [`WindowProps`] for the [`Component`]'s [`Window`].
    fn window_props() -> WindowProps;

    /// Embeds the [`Component`] into a [`Window`].
    fn windowed(self) -> Window<Self> {
        Window::with_inner(Self::window_props(), self)
    }

    /// Embeds the [`Component`] into a [`Window`] with a custom name.
    fn windowed_with_name<S>(self, name: S) -> Window<Self>
    where
        S: Into<String>,
    {
        Window::with_inner(Self::window_props().name(name), self)
    }
}
