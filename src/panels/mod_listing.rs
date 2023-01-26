use eframe::egui::{
    widgets::Widget,
    Ui,
    Response,
};

#[derive(Debug, Clone, Default)]
pub struct ModListing {
    items: Vec<ModListingItem>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
struct ModListingItem {
    pub package_id: String,
}

impl From<&crate::ModList> for ModListing {
    fn from(mod_list: &crate::ModList) -> Self {
        let items: Vec<ModListingItem> = mod_list.mods.iter()
            .map(|m| ModListingItem::from(m.1))
            .collect();

        Self { items, }
    }
}

impl From<Vec<String>> for ModListing {
    fn from(mods: Vec<String>) -> Self {
        Self {
            items: mods.into_iter()
                    .map(ModListingItem::from)
                    .collect::<Vec<ModListingItem>>()
        }
    }
}

impl From<&crate::ModMetaData> for ModListingItem {
    fn from(m: &crate::ModMetaData) -> Self {
        let package_id = m.packageId.clone().unwrap_or_default();

        Self {
            package_id,
        }
    }
}

impl From<String> for ModListingItem {
    fn from(package_id: String) -> Self {
        Self {
            package_id,
        }
    }
}

impl Widget for &ModListingItem {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(&self.package_id)
    }
}

impl Widget for &ModListing {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.scope(|ui| {
            for item in &self.items {
                ui.add(item);
            }
        }).response
    }
}

