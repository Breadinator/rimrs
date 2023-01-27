use eframe::egui::{
    widgets::Widget,
    Ui,
    Response,
};
use super::ModListingItem;

#[derive(Debug, Clone, Default)]
pub struct ModListing {
    items: Vec<ModListingItem>,
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

impl Widget for &ModListing {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.scope(|ui| {
            for item in &self.items {
                ui.add(item);
            }
        }).response
    }
}

