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
pub use helpers::traits;
pub mod serialization;
pub mod widgets;
pub mod writer_thread;
pub mod validator_thread;

// mod forward reexports
mod mods;
pub use mods::*;

// standalone reexports
pub use serialization::{
    rimpy_config::RimPyConfig,
    mods_config::ModsConfig,
};

// local imports
use panels::panel_using_widget;
use helpers::AtomicFlag;
use std::sync::{
    Arc,
    mpsc::{
        sync_channel,
        SyncSender,
    },
};
use once_cell::sync::Lazy;

const CHANNEL_BUFFER: usize = 32;

pub static CHANGED_ACTIVE_MODS: Lazy<AtomicFlag> = Lazy::new(AtomicFlag::new);

#[non_exhaustive]
#[derive(Debug)]
pub struct RimRs<'a> {
    pub rimpy_config: Arc<RimPyConfig>,
    pub mods_config: Arc<ModsConfig>,
    paths_panel: panels::PathsPanel,
    hint_panel: panels::HintPanel,
    mods_panel: panels::ModsPanel<'a>,
}

impl<'a> RimRs<'a> {
    /// Creates a new [`RimRs`] app instance.
    ///
    /// # Panics
    /// * If it fails to read [`RimPyConfig`]
    /// * If it can't read the initial mod folders
    #[must_use]
    #[allow(unused_variables, clippy::needless_pass_by_value)]
    pub fn new(cc: &CreationContext<'_>, writer_thread_tx: SyncSender<writer_thread::Message>) -> Self {
        let (hint_tx, hint_rx) = sync_channel(3);
        let hint_panel = panels::HintPanel::new(hint_rx);

        let rimpy_config = RimPyConfig::from_file().unwrap();
        let mod_list = ModList::try_from(&rimpy_config).unwrap();
        let rimpy_config = Arc::new(rimpy_config);

        let mut exe_path = rimpy_config.folders.game_folder.clone().unwrap();
        exe_path.push("RimWorldWin64.exe"); // TODO: allow for 32 bit, and also non-windows

        let cmd_args = rimpy_config.startup_params.clone();

        let mut mods_config_path = rimpy_config.folders.config_folder.clone()
            .expect("Game config folder not found in RimPy `config.ini`");
        mods_config_path.push("ModsConfig.xml");
        writer_thread_tx.send(writer_thread::Message::SetDestination(mods_config_path.clone())).expect("Couldn't setup writer thread");

        let mods_config = Arc::from(ModsConfig::try_from(mods_config_path.as_path()).unwrap());
        writer_thread_tx.send(writer_thread::Message::SetModsConfig(mods_config.clone())).expect("Couldn't setup writer thread");

        let version = mods_config.version.clone().unwrap_or(String::from("???"));

        let paths_panel = panels::PathsPanel::new(rimpy_config.clone(), version, hint_tx.clone());
        let mods_panel = panels::ModsPanel::new::<CHANNEL_BUFFER>(
            rimpy_config.clone(),
            mods_config.clone(),
            mod_list,
            &hint_tx,
            writer_thread_tx,
            exe_path,
            cmd_args,
        );

        Self { rimpy_config, mods_config, paths_panel, hint_panel, mods_panel }
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

impl<'a> App for RimRs<'a> {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("paths_panel").show(ctx, |ui| panel_using_widget(ui, &mut self.paths_panel));
        TopBottomPanel::bottom("hint_panel").show(ctx, |ui| panel_using_widget(ui, &mut self.hint_panel));
        CentralPanel::default().show(ctx, |ui| panel_using_widget(ui, &mut self.mods_panel));
    }
}

