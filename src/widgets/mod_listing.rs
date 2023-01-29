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
    pub items: Vec<ModListingItem>,
    pub title: Option<String>,
}

impl ModListing {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        mods: Vec<String>,
        mod_meta_data: &Arc<Mutex<HashMap<String, ModMetaData>>>,
        selected: &Arc<Mutex<Option<String>>>,
        title: Option<String>,
    ) -> Self {
        let id = format!("modlisting{}", fetch_inc_id(&MOD_LISTING_COUNTER));
        let items = mods.into_iter()
            .map(|m| ModListingItem::new(m, mod_meta_data.clone(), selected.clone()))
            .collect();

        Self { id, items, title }
    }

    #[must_use]
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    #[must_use]
    pub fn with_items(mut self, items: Vec<ModListingItem>) -> Self {
        self.items = items;
        self
    }

    #[must_use]
    pub fn with_mods(
        self,
        package_ids: Vec<String>,
        mod_meta_data: &Arc<Mutex<HashMap<String, ModMetaData>>>,
        selected: &Arc<Mutex<Option<String>>>,
    ) -> Self {
        let items = package_ids.into_iter()
            .map(|m| ModListingItem::new(m, mod_meta_data.clone(), selected.clone()))
            .collect();
        self.with_items(items)
    }
}

impl Widget for &ModListing {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.push_id(self.id.as_str(), |ui| {
            if let Some(title) = self.title.as_ref() {
                ui.heading(title);
            }

            let scroll_area = ScrollArea::vertical();
            scroll_area.show(ui, |ui| {
                for item in &self.items {
                    ui.add(item);
                }
            });
        }).response
    }
}

