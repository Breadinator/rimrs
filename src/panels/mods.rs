use eframe::egui::{
    Ui,
    Widget,
};
use egui_extras::{
    TableBuilder,
    Column,
};
use crate::{
    ModList,
    ModMetaData,
    panels::ModListing,
    ModsConfig,
    RimPyConfig,
};
use std::sync::{
    Arc,
    RwLock,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModsPanel {
    pub mmd: ModList,
    inactive: ModListing,
    active: ModListing,
    mod_info_widget: ModInfoWidget,
    rimpy_config: Arc<RimPyConfig>,
    mods_config: Arc<RwLock<ModsConfig>>,
}

impl ModsPanel {
    pub fn new(rimpy_config: Arc<RimPyConfig>, mods_config: Arc<RwLock<ModsConfig>>) -> Self {
        let active = ModListing::from(mods_config.read().map_or_else(|_| Vec::new(), |mc| mc.activeMods.clone()));

        Self {
            mmd: ModList::default(),
            inactive: ModListing::default(),
            active,
            mod_info_widget: ModInfoWidget::default(),
            rimpy_config,
            mods_config,
        }
    }
}

impl Widget for &mut ModsPanel {
    fn ui(self, ui: &mut Ui) -> eframe::egui::Response {
        let scope = ui.scope(|ui| { let w = ui.available_width() / 10.0;
            let mod_info_width = 4.0 * w;
            let mod_listing_width = 2.5 * w;
            let h = ui.available_height();

            TableBuilder::new(ui)
                .column(Column::exact(mod_info_width))
                .column(Column::exact(mod_listing_width))
                .column(Column::exact(mod_listing_width))
                .column(Column::remainder())
                .body(|mut body| body.row(h, |mut row| {
                    row.col(|ui| {ui.add(&self.mod_info_widget);});
                    row.col(|ui| {ui.add(&self.inactive);});
                    row.col(|ui| {ui.add(&self.active);});
                    row.col(|ui| {ui.label("btns");});
                }));
        });
        scope.response
    }
}

#[derive(Debug, Default, Clone)]
struct ModInfoWidget(Option<ModMetaData>);

impl From<ModMetaData> for ModInfoWidget {
    fn from(mmd: ModMetaData) -> Self {
        Self(Some(mmd))
    }
}

impl Widget for &ModInfoWidget {
    fn ui(self, ui: &mut Ui) -> eframe::egui::Response {
        ui.label("mod info widget")
    }
}

