use eframe::egui::{
    Widget,
    Ui,
    Response,
};

#[derive(Debug, Clone)]
pub struct ModListingItem {
    pub package_id: String,
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
