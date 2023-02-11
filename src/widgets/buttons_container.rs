use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use crate::{
    LOADED_PLUGINS,
    widgets::{
        Button,
        ModListing,
    },
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
pub struct ButtonsContainer<'a>([Option<Button<'a>>; 10]);

impl<'a> ButtonsContainer<'a> {
    /// Creates the various buttons that appear to the right of the active mods listing.
    #[must_use]
    pub fn generate(
        hint_tx: &SyncSender<String>,
        writer_thread_tx: SyncSender<crate::writer_thread::Message>,
        active_mod_listing_ref: Arc<Mutex<ModListing<'a>>>,
        exe_path: PathBuf,
        args: Option<String>,
    ) -> Self {
        let mut buttons: [Option<Button>; 10] = Default::default();
        buttons[0] = Some(Button::clear(hint_tx.clone()));
        buttons[1] = Some(Button::sort(hint_tx.clone()));
        buttons[2] = Some(Button::save(hint_tx.clone(), writer_thread_tx, active_mod_listing_ref));
        buttons[3] = Some(Button::run(hint_tx.clone(), exe_path, args));
        LOADED_PLUGINS.post_button_gen(&mut buttons);
        Self(buttons)
    }
}

impl<'a> Widget for &ButtonsContainer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.scope(|ui| {
            for btn in &self.0 {
                if let Some(btn) = btn {
                    ui.add(btn);
                }
            }
        }).response
    }
}

