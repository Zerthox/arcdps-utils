//! Window component.

use crate::{
    api::CoreColor,
    exports,
    ui::{ch_width, Component, Hideable},
};
use arcdps::imgui::{self, Condition, InputTextFlags, StyleVar, Ui};
use std::ops::{Deref, DerefMut};

/// Window options.
#[derive(Debug, Clone)]
pub struct WindowOptions {
    pub context_menu: bool,
    pub visible: bool,
    pub position: WindowPosition,
    pub width: f32,
    pub height: f32,
    pub title_bar: bool,
    pub background: bool,
    pub resize: bool,
    pub auto_resize: bool,
    pub scroll: bool,
    pub scroll_bar: bool,
}

impl WindowOptions {
    /// Creates the default window options.
    pub const fn new() -> Self {
        Self {
            context_menu: false,
            visible: true,
            position: WindowPosition::Manual,
            width: 0.0,
            height: 0.0,
            title_bar: true,
            background: true,
            resize: true,
            auto_resize: false,
            scroll: true,
            scroll_bar: true,
        }
    }
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum WindowPosition {
    Manual,
    Anchored {
        anchor: WindowAnchor,
        x: f32,
        y: f32,
    },
}

impl WindowPosition {
    /// Calculates the UI position.
    pub fn calc(&self, ui: &Ui, window_size: [f32; 2]) -> [f32; 2] {
        match self {
            Self::Manual => [0.0, 0.0],
            Self::Anchored { anchor, x, y } => {
                let [screen_x, screen_y] = ui.io().display_size;
                let [window_x, window_y] = window_size;
                let rel_x = *x;
                let rel_y = *y;

                match anchor {
                    WindowAnchor::TopLeft => [rel_x, rel_y],
                    WindowAnchor::TopRight => [screen_x - window_x - rel_x, rel_y],
                    WindowAnchor::BottomLeft => [rel_x, screen_y - window_y - rel_y],
                    WindowAnchor::BottomRight => {
                        [screen_x - window_x - rel_x, screen_y - window_y - rel_y]
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowAnchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Window component.
#[derive(Debug, Clone)]
pub struct Window<T>
where
    T: Component,
{
    pub inner: T,
    pub name: String,
    pub options: WindowOptions,
}

impl<T> Window<T>
where
    T: Component,
{
    /// Creates a new window with a given inner [`Component`].
    pub fn new(name: impl Into<String>, inner: T) -> Self {
        Self::with_options(name, inner, WindowOptions::new())
    }

    /// Creates a new window with a given inner [`Component`].
    pub fn with_options(name: impl Into<String>, inner: T, options: WindowOptions) -> Self {
        Self {
            inner,
            name: name.into(),
            options,
        }
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
        if self.options.visible {
            let inner = &mut self.inner;
            let size = [self.options.width, self.options.height];
            let pos = self.options.position.calc(ui, size);

            imgui::Window::new(&self.name)
                .size(
                    size,
                    if self.options.auto_resize {
                        Condition::FirstUseEver
                    } else {
                        Condition::Always
                    },
                )
                .position(
                    pos,
                    if self.options.position == WindowPosition::Manual {
                        Condition::FirstUseEver
                    } else {
                        Condition::Always
                    },
                )
                .collapsible(false)
                .title_bar(self.options.title_bar)
                .draw_background(self.options.background)
                .always_auto_resize(self.options.auto_resize)
                .resizable(self.options.resize)
                .scrollable(self.options.scroll)
                .scroll_bar(self.options.scroll_bar)
                .focus_on_appearing(false)
                .opened(&mut self.options.visible)
                .build(ui, || {
                    // update dimensions
                    [self.options.width, self.options.height] = ui.window_size();

                    if self.options.context_menu {
                        // render window context menu
                        window_context_menu(&format!("Options##{}", self.name), || {
                            let colors = exports::colors();
                            let grey = colors
                                .core(CoreColor::MediumGrey)
                                .unwrap_or([0.5, 0.5, 0.5, 1.0]);

                            // use small padding similar to arc & other plugins
                            let _style = ui.push_style_var(StyleVar::FramePadding([1.0, 1.0]));

                            let input_width = ch_width(ui, 12);
                            const STEP: f32 = 1.0;
                            const STEP_FAST: f32 = 10.0;
                            const FORMAT: &str = "%.0f";

                            ui.menu("Style", || {
                                ui.text_colored(grey, "Window Style");

                                ui.checkbox("Titlebar", &mut self.options.title_bar);
                                ui.checkbox("Background", &mut self.options.background);
                                ui.checkbox("Scrollbar", &mut self.options.scroll_bar);
                                ui.checkbox("Auto Resize", &mut self.options.auto_resize);

                                ui.set_next_item_width(input_width);

                                let current = ui.clone_style().alpha;
                                let _style = ui.push_style_var(StyleVar::Alpha(
                                    if self.options.auto_resize {
                                        0.3
                                    } else {
                                        current
                                    },
                                ));

                                let flags = if self.options.auto_resize {
                                    InputTextFlags::READ_ONLY
                                } else {
                                    InputTextFlags::empty()
                                };

                                input_float_with_format(
                                    "Width",
                                    &mut self.options.width,
                                    STEP,
                                    STEP_FAST,
                                    FORMAT,
                                    flags,
                                );

                                ui.set_next_item_width(input_width);
                                input_float_with_format(
                                    "Height",
                                    &mut self.options.height,
                                    STEP,
                                    STEP_FAST,
                                    FORMAT,
                                    flags,
                                );
                            });

                            ui.menu("Position", || {
                                ui.text_colored(grey, "Window Position");

                                if ui.radio_button_bool(
                                    "Manual",
                                    self.options.position == WindowPosition::Manual,
                                ) {
                                    self.options.position = WindowPosition::Manual;
                                }

                                if ui.radio_button_bool(
                                    "Screen Relative",
                                    matches!(
                                        self.options.position,
                                        WindowPosition::Anchored { .. }
                                    ),
                                ) {
                                    self.options.position = WindowPosition::Anchored {
                                        anchor: WindowAnchor::TopLeft,
                                        x: 0.0,
                                        y: 0.0,
                                    }
                                }

                                if let WindowPosition::Anchored { anchor, x, y } =
                                    &mut self.options.position
                                {
                                    ui.indent();

                                    ui.radio_button("Top Left", anchor, WindowAnchor::TopLeft);
                                    ui.radio_button("Top Right", anchor, WindowAnchor::TopRight);
                                    ui.radio_button(
                                        "Bottom Left",
                                        anchor,
                                        WindowAnchor::BottomLeft,
                                    );
                                    ui.radio_button(
                                        "Bottom Right",
                                        anchor,
                                        WindowAnchor::BottomRight,
                                    );

                                    ui.set_next_item_width(input_width);
                                    input_float_with_format(
                                        "x",
                                        x,
                                        STEP,
                                        STEP_FAST,
                                        FORMAT,
                                        InputTextFlags::empty(),
                                    );

                                    ui.set_next_item_width(input_width);
                                    input_float_with_format(
                                        "y",
                                        y,
                                        STEP,
                                        STEP_FAST,
                                        FORMAT,
                                        InputTextFlags::empty(),
                                    );

                                    ui.unindent();
                                }
                            });
                        });
                    }

                    // render window contents
                    inner.render(ui, props);
                });
        }
    }
}

impl<T> Hideable for Window<T>
where
    T: Component,
{
    fn is_visible(&self) -> bool {
        self.options.visible
    }

    fn visible_mut(&mut self) -> &mut bool {
        &mut self.options.visible
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

            if let Some(settings) = loaded.settings {
                self.inner.load_settings(settings);
            }
        }
    }
}

#[cfg(feature = "settings")]
pub use settings::*;

use super::{input_float_with_format, window_context_menu};
