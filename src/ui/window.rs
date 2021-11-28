use super::{Component, Hideable};
use arcdps::imgui::{self, Condition, ImString, Ui};
use std::ops::{Deref, DerefMut};

/// A component which may render in a window.
pub trait Windowed
where
    Self: Component + Default + Sized,
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
