use super::Updater;
use crate::{
    colors::{GREEN, YELLOW},
    ui::{Component, Ui},
};
use arcdps::imgui::Window;

impl Updater {
    /// Renders the update window if necessary.
    #[inline]
    pub fn render(&mut self, ui: &Ui) {
        if let Some(latest) = self.latest_if_outdated().cloned() {
            let mut open = true;

            Window::new(format!("{} Update", self.name))
                .always_auto_resize(true)
                .collapsible(false)
                .opened(&mut open)
                .build(ui, || {
                    ui.text_colored(YELLOW, format!("Current version: {}", self.current));
                    ui.text_colored(GREEN, format!("Latest version: {latest}"));

                    if ui.button("Open") {
                        self.repo.open_release();
                    }
                    ui.same_line();
                    if ui.button("Ignore") {
                        self.reset();
                    }
                });

            if !open {
                self.reset();
            }
        }
    }
}

impl Component<()> for Updater {
    fn render(&mut self, ui: &Ui, _: ()) {
        self.render(ui);
    }
}
