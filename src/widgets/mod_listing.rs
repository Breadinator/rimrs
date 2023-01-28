use eframe::egui::{
    Widget,
    Ui,
    Response,
    ScrollArea,
};
use crate::{
    ModMetaData,
    widgets::ModListingItem,
    helpers::fetch_inc_id,
};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
        atomic::AtomicUsize,
    },
};
use once_cell::sync::Lazy;

static MOD_LISTING_COUNTER: Lazy<Arc<AtomicUsize>> = Lazy::new(|| Arc::new(AtomicUsize::new(0usize)));

#[derive(Debug, Clone, Default)]
pub struct ModListing {
    id: String,
    items: Vec<ModListingItem>,
}

impl ModListing {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        mods: Vec<String>,
        mod_meta_data: Arc<Mutex<HashMap<String, ModMetaData>>>,
        selected: Arc<Mutex<Option<String>>>,
    ) -> Self {
        let id = format!("modlisting{}", fetch_inc_id(&MOD_LISTING_COUNTER));
        let items = mods.into_iter()
            .map(|m| ModListingItem::new(m, mod_meta_data.clone(), selected.clone()))
            .collect();

        Self { id, items }
    }
}

impl Widget for &ModListing {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.push_id(self.id.as_str(), |ui| {
            let scroll_area = ScrollArea::vertical();
            scroll_area.show(ui, |ui| {
                for item in &self.items {
                    ui.add(item);
                }
            });
        }).response
    }
}

