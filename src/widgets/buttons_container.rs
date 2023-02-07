use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use super::Button;
use std::sync::mpsc::SyncSender;

#[derive(Debug)]
pub struct ButtonsContainer<'a>(Vec<Button<'a>>);

impl ButtonsContainer<'_> {
    /// Creates the various buttons that appear to the right of the active mods listing.
    #[must_use]
    pub fn generate(hint_tx: SyncSender<String>) -> Self {
        Self(vec![
             Button::clear(hint_tx.clone()),
             Button::sort(hint_tx.clone()),
             Button::save(hint_tx.clone()),
             Button::run(hint_tx),
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

