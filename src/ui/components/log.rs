//! Log component.

use crate::ui::{align::RightAlign, Component, WindowProps, Windowed};
use arcdps::imgui::{im_str, ChildWindow, ImString, Ui};
use chrono::Local;

/// Time format used for log messages.
const FORMAT: &str = "%b %d %H:%M:%S.%3f";

/// Component for logging messages.
#[derive(Debug, Clone)]
pub struct Log {
    /// Current contents of the log.
    pub contents: ImString,

    /// Whether the log is active.
    pub active: bool,

    /// Last maximum scroll position.
    last_scroll_max: f32,

    // button widths used for ui rendering
    activity_toggle_width: f32,
    clear_button_width: f32,
    copy_button_width: f32,
}

impl Log {
    /// Creates a new log.
    pub fn new() -> Self {
        Self {
            contents: ImString::default(),
            active: true,
            last_scroll_max: 0.0,
            activity_toggle_width: 60.0,
            clear_button_width: 60.0,
            copy_button_width: 60.0,
        }
    }

    /// Appends output to the log.
    pub fn log<S>(&mut self, output: S)
    where
        S: AsRef<str>,
    {
        if self.active {
            // generate line
            let now = Local::now();
            let line = format!("{}: {}\n", now.format(FORMAT), output.as_ref());

            // append line
            self.contents.push_str(&line);
        }
    }

    /// Clears the log.
    pub fn clear(&mut self) {
        self.contents.clear();
    }
}

impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Log {
    fn render(&mut self, ui: &Ui) {
        // time
        ui.align_text_to_frame_padding();
        ui.text(format!("Time: {}", Local::now().format(FORMAT)));

        // buttons from right to left
        let mut align = RightAlign::build();

        // clear button
        let contents = &mut self.contents;
        align.item(ui, &mut self.clear_button_width, || {
            if ui.button(im_str!("Clear"), [0.0, 0.0]) {
                contents.clear();
            }
        });

        // copy button
        align.item(ui, &mut self.copy_button_width, || {
            if ui.button(im_str!("Copy"), [0.0, 0.0]) {
                ui.set_clipboard_text(contents);
            }
        });

        // activity toggle
        let active = &mut self.active;
        align.item_with_margin(ui, 10.0, &mut self.activity_toggle_width, || {
            ui.checkbox(im_str!("Active"), active);
        });

        ui.separator();

        // log contents
        ChildWindow::new(im_str!("##log-scroller"))
            .scrollable(true)
            .horizontal_scrollbar(true)
            .build(ui, || {
                // render text
                ui.text(&self.contents);

                // perform auto scroll
                // strict comparison should fine with the values returned by imgui
                #[allow(clippy::float_cmp)]
                if ui.scroll_y() == self.last_scroll_max {
                    ui.set_scroll_here_y_with_ratio(1.0);
                }

                // update last max
                self.last_scroll_max = ui.scroll_max_y();
            })
    }
}

impl Windowed for Log {
    fn window_props() -> WindowProps {
        WindowProps::new("Log")
            .visible(true)
            .width(600.0)
            .height(300.0)
    }
}
