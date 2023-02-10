use crate::{
    ModMetaData,
    widgets::ModListingItem,
    traits::TableRower,
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
use eframe::egui::{
    Widget,
    Ui,
    Response,
    Layout,
    Align,
};

/// A single list of mods.
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
        const BUTTON_WIDTH: f32 = 16.0;
        const ROW_HEIGHT: f32 = 16.0;

        ui.push_id(self.id.as_str(), |ui| {
            if let Some(title) = self.title.as_ref() {
                ui.heading(title);
            }

            egui_extras::TableBuilder::new(ui)
                .column(egui_extras::Column::exact(BUTTON_WIDTH))
                .column(egui_extras::Column::exact(BUTTON_WIDTH))
                .column(egui_extras::Column::remainder())
                .cell_layout(Layout::left_to_right(Align::Min).with_main_wrap(false))
                .body(|mut body| {
                    for item in &self.items {
                        body.row(ROW_HEIGHT, |r| item.table_row(r));
                    }
                });
        }).response
    }
}

impl From<&ModListing<'_>> for Vec<String> {
    fn from(mod_listing: &ModListing<'_>) -> Self {
        mod_listing.items.iter()
            .map(|item| item.package_id.clone())
            .collect()
    }
}

