use arcdps::imgui::{Selectable, StyleColor, Ui};
use std::borrow::Cow;
use strum::{IntoEnumIterator, VariantArray};

/// Renders a combo box for items from an iterator.
// TODO: make more generic?
pub fn combo<T>(
    ui: &Ui,
    label: impl AsRef<str>,
    all: impl IntoIterator<Item = T>,
    current: &mut T,
    item_label: impl Fn(&T) -> Cow<str>,
    item_color: impl Fn(&T) -> Option<[f32; 4]>,
) -> bool
where
    T: PartialEq,
{
    let mut changed = false;
    if let Some(_token) = ui.begin_combo(label, item_label(current).as_ref()) {
        for entry in all {
            let _style =
                item_color(&entry).map(|color| ui.push_style_color(StyleColor::Text, color));
            let selected = entry == *current;
            if Selectable::new(item_label(&entry).as_ref())
                .selected(selected)
                .build(ui)
            {
                changed = true;
                *current = entry;
            }
            if selected {
                ui.set_item_default_focus();
            }
        }
    }
    changed
}

/// Renders a combo box for an enum implementing [`IntoEnumIterator`].
pub fn enum_combo<T>(ui: &Ui, label: impl AsRef<str>, current: &mut T) -> bool
where
    T: PartialEq + AsRef<str> + IntoEnumIterator,
{
    let mut changed = false;
    if let Some(_token) = ui.begin_combo(label, &current) {
        for entry in T::iter() {
            let selected = entry == *current;
            if Selectable::new(&entry).selected(selected).build(ui) {
                changed = true;
                *current = entry;
            }
            if selected {
                ui.set_item_default_focus();
            }
        }
    }
    changed
}

/// Renders a combo box for an enum implementing [`VariantArray`].
pub fn enum_combo_array<T>(ui: &Ui, label: impl AsRef<str>, current: &mut T) -> bool
where
    T: Clone + PartialEq + AsRef<str> + VariantArray,
{
    let mut changed = false;
    if let Some(_token) = ui.begin_combo(label, &current) {
        for entry in T::VARIANTS {
            let selected = *entry == *current;
            if Selectable::new(&entry).selected(selected).build(ui) {
                changed = true;
                *current = entry.clone();
            }
            if selected {
                ui.set_item_default_focus();
            }
        }
    }
    changed
}
