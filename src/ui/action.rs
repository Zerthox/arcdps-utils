use arcdps::imgui::{StyleVar, Ui};

/// An action for ordered lists in UI.
///
/// # Examples
/// ```no_run
/// use arc_util::ui::action::Action;
///
/// # let ui: &arcdps::imgui::Ui = todo!();
/// let mut vec = vec!["a", "b", "c"];
/// let mut action = Action::new();
/// for (i, entry) in vec.iter().enumerate() {
///     ui.text(entry);
///     ui.same_line();
///     action.render_buttons(ui, "buttons-id", i, vec.len());
/// }
/// action.perform(&mut vec);
/// ```
#[derive(Debug, Default, Clone)]
pub enum Action {
    /// No action to be performed.
    #[default]
    None,

    /// Swap index with next element.
    Swap(usize),

    /// Delete index.
    Remove(usize),
}

impl Action {
    /// Creates a new [`Action`].
    pub const fn new() -> Self {
        Self::None
    }

    /// Renders action buttons.
    pub fn render_buttons(&mut self, ui: &Ui, id: impl AsRef<str>, index: usize, len: usize) {
        let id = id.as_ref();
        let is_first = index == 0;
        let is_last = index == len - 1;
        let current_alpha = ui.clone_style().alpha;

        let style = ui.push_style_var(StyleVar::Alpha(if is_first { 0.3 } else { current_alpha }));
        if ui.button(format!("^##{id}-{index}")) && !is_first {
            *self = Self::Swap(index - 1);
        }
        style.pop();

        ui.same_line();
        let style = ui.push_style_var(StyleVar::Alpha(if is_last { 0.3 } else { current_alpha }));
        if ui.button(format!("v##{id}-{index}")) && !is_last {
            *self = Self::Swap(index);
        }
        style.pop();

        ui.same_line();
        if ui.button(format!("x##{id}-{index}")) {
            *self = Self::Remove(index);
        }
    }

    /// Performs the action on a [`Vec`].
    pub fn perform<T>(&self, vec: &mut Vec<T>) {
        match *self {
            Self::None => {}
            Self::Swap(i) => vec.swap(i, i + 1),
            Self::Remove(i) => {
                vec.remove(i);
            }
        }
    }
}
