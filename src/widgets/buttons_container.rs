use crate::{
    widgets::{Button, ModListing},
    ModMetaData,
};
use eframe::egui::{Response, Ui, Widget};
use std::{
    cell::RefCell,
    collections::HashMap,
    path::PathBuf,
    rc::Rc,
    sync::{
        mpsc::{Sender, SyncSender},
        Arc, Mutex,
    },
};

/// Wrapper for the [`Vec`] of [`Button`]s that appear to the right of the active mods list.
#[derive(Debug)]
pub struct ButtonsContainer<'a>(Vec<Button<'a>>);

impl<'a> ButtonsContainer<'a> {
    /// Creates the various buttons that appear to the right of the active mods listing.
    #[must_use]
    pub fn generate(
        hint_tx: &SyncSender<String>,
        writer_thread_tx: SyncSender<crate::writer_thread::Message>,
        change_mod_list_tx: Sender<Vec<String>>,
        active_mod_listing_ref: Rc<RefCell<ModListing<'a>>>,
        mod_meta_data: Arc<Mutex<HashMap<String, ModMetaData>>>,
        exe_path: PathBuf,
        args: Option<String>,
    ) -> Self {
        Self(vec![
            Button::clear(hint_tx.clone(), change_mod_list_tx.clone()),
            Button::sort(
                hint_tx.clone(),
                change_mod_list_tx.clone(),
                active_mod_listing_ref.clone(),
                mod_meta_data,
            ),
            Button::import_list(hint_tx.clone(), change_mod_list_tx),
            Button::export_list(
                hint_tx.clone(),
                writer_thread_tx.clone(),
                active_mod_listing_ref.clone(),
            ),
            Button::save(hint_tx.clone(), writer_thread_tx, active_mod_listing_ref),
            Button::run(hint_tx.clone(), exe_path, args),
        ])
    }
}

impl<'a> Widget for &ButtonsContainer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.scope(|ui| {
            for btn in &self.0 {
                ui.add(btn);
            }
        })
        .response
    }
}
