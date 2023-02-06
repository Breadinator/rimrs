use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use super::Button;

#[derive(Debug)]
pub struct ButtonsContainer<'a>(Vec<Button<'a>>);

impl ButtonsContainer<'_> {
    /// Creates the various buttons that appear to the right of the active mods listing.
    #[must_use]
    pub fn generate() -> Self {
        Self::default()
    }
}

impl Default for ButtonsContainer<'_> {
    fn default() -> Self {
        Self(vec![
             Button::clear(),
             Button::sort(),
             Button::save(),
             Button::run(),
        ])
    }
}

impl<'a> Widget for &ButtonsContainer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.scope(|ui| {
            for btn in &self.0 {
                ui.add(btn);
            }
        }).response
    }
}

