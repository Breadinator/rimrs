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
pub mod widgets;

// mod forward reexports
mod mods;
pub use mods::*;

// local imports
use panels::panel_using_widget;
use serialization::{
    rimpy_config::RimPyConfig,
    mods_config::ModsConfig,
};
use std::sync::{Arc, RwLock};

const CHANNEL_BUFFER: usize = 32;

#[non_exhaustive]
#[derive(Debug)]
pub struct RimRs<'a> {
    pub rimpy_config: Arc<RimPyConfig>,
    pub mods_config: Arc<RwLock<ModsConfig>>,
    paths_panel: panels::PathsPanel,
    hint_panel: panels::HintPanel,
    mods_panel: panels::ModsPanel<'a>,
}

impl RimRs<'_> {
    /// Creates a new [`RimRs`] app instance.
    ///
    /// # Panics
    /// * If it fails to read [`RimPyConfig`]
    /// * If it can't read the initial mod folders
    #[must_use]
    #[allow(unused_variables)]
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let rimpy_config_unwrapped = RimPyConfig::from_file().unwrap();
        let mod_list = ModList::try_from(&rimpy_config_unwrapped).unwrap();
        let rimpy_config = Arc::new(rimpy_config_unwrapped);

        let mut mods_config_path = rimpy_config.folders.config_folder.clone()
            .expect("Game config folder not found in RimPy `config.ini`");
        mods_config_path.push("ModsConfig.xml");
        let mods_config = Arc::from(RwLock::from(ModsConfig::try_from(mods_config_path.as_path()).unwrap()));

        let version = mods_config.read().unwrap().version.clone().unwrap_or(String::from("???"));

        let paths_panel = panels::PathsPanel::new(rimpy_config.clone(), version);
        let hint_panel = panels::HintPanel::default();
        let mods_panel = panels::ModsPanel::new::<CHANNEL_BUFFER>(rimpy_config.clone(), mods_config.clone(), mod_list);

        Self {
            rimpy_config,
            mods_config,
            paths_panel,
            hint_panel,
            mods_panel,
        }
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
            Ok(mod_list) => self.mods_panel.mods = mod_list,
            Err(e) => log::error!("{e}"),
        }
    }
}

impl App for RimRs<'_> {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("paths_panel").show(ctx, |ui| panel_using_widget(ui, &mut self.paths_panel));
        TopBottomPanel::bottom("hint_panel").show(ctx, |ui| panel_using_widget(ui, &self.hint_panel));
        CentralPanel::default().show(ctx, |ui| panel_using_widget(ui, &mut self.mods_panel));
    }
}

