use eframe::egui::{
    Widget,
    SelectableLabel,
    Ui,
    Response,
};
use crate::ModMetaData;
use std::{
    sync::{
        Arc,
        Mutex,
        mpsc::SyncSender,
    },
    collections::HashMap,
};

#[derive(Debug, Clone)]
pub struct ModListingItem {
    pub package_id: String,
    pub mod_meta_data: Option<Arc<Mutex<HashMap<String, ModMetaData>>>>,
    pub selected: Arc<Mutex<Option<String>>>,
    // pub tx: SyncSender<String>,
}

impl ModListingItem {
    #[must_use]
    pub fn new(
        package_id: String,
        mod_meta_data: Arc<Mutex<HashMap<String, ModMetaData>>>,
        selected: Arc<Mutex<Option<String>>>,
    ) -> Self {
        Self {
            package_id,
            mod_meta_data: Some(mod_meta_data),
            selected,
        }
    }

    fn get_display_name(&self) -> String {
        if let Some(mmd_mutex) = &self.mod_meta_data {
            if let Ok(mmd) = mmd_mutex.lock() {
                if let Some(package_mmd) = mmd.get(&self.package_id) {
                    if let Some(name) = package_mmd.name.clone() {
                        return name;
                    }
                }
            }
        }

        self.package_id.clone()
    }
}

impl From<String> for ModListingItem {
    fn from(package_id: String) -> Self {
        Self {
            package_id,
            mod_meta_data: None,
            selected: crate::helpers::arc_mutex_none(),
        }
    }
}

impl Widget for &ModListingItem {
    fn ui(self, ui: &mut Ui) -> Response {
        if let Ok(mut sel) = self.selected.try_lock() {
            let is_selected = (*sel).clone().map_or(false, |sel| self.package_id == sel);
            let lab = ui.add(SelectableLabel::new(is_selected, self.get_display_name()));

            if lab.clicked() {
                *sel = Some(self.package_id.clone().to_lowercase());
            }

            lab
        } else {
            ui.selectable_label(false, self.get_display_name())
        }
    }
}

