use eframe::egui::{
    Ui,
    Widget,
};
use crate::{
    ModList,
    panels::ModListing,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ModsPanel {
    pub mmd: ModList,
    inactive: ModListing,
    active: ModListing,
}

impl Widget for &mut ModsPanel {
    fn ui(self, ui: &mut Ui) -> eframe::egui::Response {
        let group = ui.group(|ui| {
            ui.add(&self.inactive);
            // ui.label("Mods panel");
        });
        group.response
    }
}

