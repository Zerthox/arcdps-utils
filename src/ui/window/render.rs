use super::{WindowOptions, WindowPosition};
use crate::colors::TRANSPARENT;
use arcdps::imgui::{Condition, StyleColor, Ui, Window, WindowToken};

impl WindowOptions {
    /// Renders the window according to the options.
    pub fn render_window<'ui>(&mut self, ui: &'ui Ui, name: &str) -> Option<WindowToken<'ui>> {
        if self.visible {
            let size = [self.width, self.height];
            let pos = self.position.calc(ui, size);

            let _style = if !self.title_bar_background {
                Some((
                    ui.push_style_color(StyleColor::TitleBg, TRANSPARENT),
                    ui.push_style_color(StyleColor::TitleBgActive, TRANSPARENT),
                    ui.push_style_color(StyleColor::TitleBgCollapsed, TRANSPARENT),
                ))
            } else {
                None
            };

            Window::new(&name)
                .size(
                    size,
                    if self.auto_resize {
                        Condition::Never
                    } else {
                        Condition::Always
                    },
                )
                .position(
                    pos.unwrap_or_default(),
                    if let WindowPosition::Manual = self.position {
                        Condition::FirstUseEver
                    } else {
                        Condition::Always
                    },
                )
                .collapsible(false)
                .title_bar(self.title_bar)
                .draw_background(self.background)
                .always_auto_resize(self.auto_resize)
                .resizable(self.resize)
                .scrollable(self.scroll)
                .scroll_bar(self.scroll_bar)
                .focus_on_appearing(false)
                .opened(&mut self.visible)
                .begin(ui)
        } else {
            None
        }
    }
}
