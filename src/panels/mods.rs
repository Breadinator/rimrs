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
    ModsConfig,
    RimPyConfig,
    widgets::{
        ModListing,
        ModInfoWidget,
        Btns,
    },
};
use std::sync::{
    Arc,
    RwLock,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ModsPanel<'a> {
    pub mmd: ModList,
    inactive: ModListing,
    active: ModListing,
    mod_info_widget: ModInfoWidget,
    rimpy_config: Arc<RimPyConfig>,
    mods_config: Arc<RwLock<ModsConfig>>,
    btns: Btns<'a>,
}

impl ModsPanel<'_> {
    pub fn new(rimpy_config: Arc<RimPyConfig>, mods_config: Arc<RwLock<ModsConfig>>) -> Self {
        let active = ModListing::from(mods_config.read().map_or_else(|_| Vec::new(), |mc| mc.activeMods.clone()));

        Self {
            mmd: ModList::default(),
            inactive: ModListing::default(),
            active,
            mod_info_widget: ModInfoWidget::default(),
            rimpy_config,
            mods_config,
            btns: Btns::default(),
        }
    }
}

impl Widget for &mut ModsPanel<'_> {
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
                    row.col(|ui| {ui.add(&self.btns);});
                }));
        });
        scope.response
    }
}

