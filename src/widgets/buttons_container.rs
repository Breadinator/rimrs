use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use crate::widgets::{
    Button,
    ModListing,
};
use std::{
    sync::{
        Arc,
        Mutex,
        mpsc::SyncSender,
    },
    path::PathBuf,
};

/// Wrapper for the [`Vec`] of [`Button`]s that appear to the right of the active mods list.
#[derive(Debug)]
pub struct ButtonsContainer<'a>(Vec<Button<'a>>);

impl<'a> ButtonsContainer<'a> {
    /// Creates the various buttons that appear to the right of the active mods listing.
    #[must_use]
    pub fn generate(
        hint_tx: SyncSender<String>,
        writer_thread_tx: SyncSender<crate::writer_thread::Message>,
        change_mod_list_tx: SyncSender<Vec<String>>,
        active_mod_listing_ref: Arc<Mutex<ModListing<'a>>>,
        exe_path: PathBuf,
        args: Option<String>,
    ) -> Self {
        Self(vec![
             Button::clear(hint_tx.clone()),
             Button::sort(hint_tx.clone()),
             Button::save(hint_tx.clone(), writer_thread_tx, active_mod_listing_ref),
             Button::run(hint_tx.clone(), exe_path, args),
             Button::import_list(hint_tx, change_mod_list_tx),
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

