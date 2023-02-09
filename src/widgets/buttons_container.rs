use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use crate::widgets::{
    Button,
    ModListing,
};
use std::sync::{
    Arc,
    Mutex,
    mpsc::SyncSender,
};

#[derive(Debug)]
pub struct ButtonsContainer<'a>(Vec<Button<'a>>);

impl<'a> ButtonsContainer<'a> {
    /// Creates the various buttons that appear to the right of the active mods listing.
    #[must_use]
    pub fn generate(hint_tx: SyncSender<String>, writer_thread_tx: SyncSender<crate::writer_thread::Message>, active_mod_listing_ref: Arc<Mutex<ModListing<'a>>>) -> Self {
        Self(vec![
             Button::clear(hint_tx.clone()),
             Button::sort(hint_tx.clone()),
             Button::save(hint_tx.clone(), writer_thread_tx, active_mod_listing_ref),
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

