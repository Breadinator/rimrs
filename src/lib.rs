#![warn(clippy::pedantic)]

use eframe::{
    egui::{
        self,
        CentralPanel,
        TopBottomPanel,
    },
    CreationContext,
    App,
};

// pub mods
pub mod panels;
pub mod helpers;
pub mod serialization;

// mod forward reexports
mod mods;
pub use mods::*;

// local imports
use panels::panel_using_widget;
use serialization::RimPyConfig;

#[derive(Debug)]
pub struct RimRs {
    rimpy_config: RimPyConfig,
    paths_panel: panels::PathsPanel,
    hint_panel: panels::HintPanel,
    mods_panel: panels::ModsPanel,
}

impl RimRs {
    /// Creates a new [`RimRs`] app instance.
    ///
    /// # Panics
    /// * If it fails to read [`RimPyConfig`]
    #[must_use]
    #[allow(unused_variables)]
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let conf = RimPyConfig::from_file().unwrap();

        let mut s = Self {
            rimpy_config: conf,
            paths_panel: panels::PathsPanel::default(),
            hint_panel: panels::HintPanel::default(),
            mods_panel: panels::ModsPanel::default(),
        };

        s.update_modlist();

        s
    }

    pub fn update_modlist(&mut self) {
        let mut paths = Vec::new();
        if let Some(p) = &self.rimpy_config.folders.expansions {
            paths.push(p.clone());
        }
        if let Some(p) = &self.rimpy_config.folders.steam_mods {
            paths.push(p.clone());
        }
        if let Some(p) = &self.rimpy_config.folders.local_mods {
            paths.push(p.clone());
        }

        match ModList::from_dirs(paths) {
            Ok(mod_list) => self.mods_panel.mmd = mod_list,
            Err(e) => log::error!("{e}"),
        }
    }
}

impl App for RimRs {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("paths_panel").show(ctx, |ui| panel_using_widget(ui, &mut self.paths_panel));
        TopBottomPanel::bottom("hint_panel").show(ctx, |ui| panel_using_widget(ui, &self.hint_panel));
        CentralPanel::default().show(ctx, |ui| panel_using_widget(ui, &mut self.mods_panel));
    }
}

