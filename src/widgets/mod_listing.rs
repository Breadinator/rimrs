use eframe::egui::{
    Widget,
    Ui,
    Response,
    ScrollArea,
};
use crate::{
    ModMetaData,
    widgets::ModListingItem,
    helpers::{
        fetch_inc_id,
        vec_ops::MultiVecOp,
    },
};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
        mpsc::SyncSender,
    },
};

#[derive(Debug, Clone)]
pub struct ModListing<'a> {
    id: String,
    pub items: Vec<ModListingItem<'a>>,
    pub title: Option<String>,
    #[allow(dead_code)] // just holding onto it to avoid TryRecvError::Disconnected
    tx: SyncSender<MultiVecOp<'a, ModListingItem<'a>>>,
}

impl<'a> ModListing<'a> {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        mods: Vec<String>,
        mod_meta_data: &Arc<Mutex<HashMap<String, ModMetaData>>>,
        selected: &Arc<Mutex<Option<String>>>,
        title: Option<String>,
        tx: SyncSender<MultiVecOp<'a, ModListingItem<'a>>>,
    ) -> Self {
        let id = format!("modlisting{}", fetch_inc_id());
        let items = mods.into_iter()
            .map(|m| ModListingItem::new(m, mod_meta_data.clone(), selected.clone(), tx.clone()))
            .collect();

        Self { id, items, title, tx }
    }

    #[must_use]
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    #[must_use]
    pub fn with_items(mut self, items: Vec<ModListingItem<'a>>) -> Self {
        self.items = items;
        self
    }

    #[must_use]
    pub fn with_mods(
        self,
        package_ids: Vec<String>,
        mod_meta_data: &Arc<Mutex<HashMap<String, ModMetaData>>>,
        selected: &Arc<Mutex<Option<String>>>,
        tx: &SyncSender<MultiVecOp<'a, ModListingItem<'a>>>,
    ) -> Self {
        let items = package_ids.into_iter()
            .map(|m| ModListingItem::new(m, mod_meta_data.clone(), selected.clone(), tx.clone()))
            .collect();
        self.with_items(items)
    }
}

impl Widget for &ModListing<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.push_id(self.id.as_str(), |ui| {
            if let Some(title) = self.title.as_ref() {
                ui.heading(title);
            }

            ScrollArea::vertical()
                .show(ui, |ui| {
                    for item in &self.items {
                        ui.add(item);
                    }
                });
        }).response
    }
}

