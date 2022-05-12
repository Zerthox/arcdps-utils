use super::{WindowOptions, WindowPosition};
use arcdps::imgui::{Condition, Ui, Window, WindowToken};

/// Renders a window.
pub fn render_window<'ui>(ui: &'ui Ui, options: &mut WindowOptions) -> Option<WindowToken<'ui>> {
    if options.visible {
        let size = [options.width, options.height];
        let pos = options.position.calc(ui, size);

        Window::new(&options.name)
            .size(
                size,
                if options.auto_resize {
                    Condition::FirstUseEver
                } else {
                    Condition::Always
                },
            )
            .position(
                pos,
                if options.position == WindowPosition::Manual {
                    Condition::FirstUseEver
                } else {
                    Condition::Always
                },
            )
            .collapsible(false)
            .title_bar(options.title_bar)
            .draw_background(options.background)
            .always_auto_resize(options.auto_resize)
            .resizable(options.resize)
            .scrollable(options.scroll)
            .scroll_bar(options.scroll_bar)
            .focus_on_appearing(false)
            .opened(&mut options.visible)
            .begin(ui)
    } else {
        None
    }
}
