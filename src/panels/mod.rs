#![allow(clippy::module_name_repetitions)]

use eframe::egui::{Ui, Widget};

mod paths;
pub use paths::*;

mod mods;
pub use mods::*;

mod hint;
pub use hint::*;

pub fn panel_using_widget(ui: &mut Ui, widget: impl Widget) {
    ui.add(widget);
}
