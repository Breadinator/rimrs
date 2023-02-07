use eframe::egui::{
    Ui,
    Response,
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
        ModListingItem,
        ModInfo,
        ButtonsContainer,
    },
    helpers::{
        arc_mutex_none,
        traits::LogIfErr,
        vec_ops::MultiVecOp,
    },
};
use std::sync::{
    Arc,
    RwLock,
    mpsc::{
        sync_channel,
        SyncSender,
        Receiver,
        TryRecvError,
    },
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ModsPanel<'a> {
    pub mods: ModList,
    inactive: ModListing<'a>,
    active: ModListing<'a>,
    mod_info_widget: ModInfo,
    btns: ButtonsContainer<'a>,
    rimpy_config: Arc<RimPyConfig>,
    mods_config: Arc<RwLock<ModsConfig>>,
    rx: Receiver<MultiVecOp<'a, ModListingItem<'a>>>,
}

impl ModsPanel<'_> {
    pub fn new<const SIZE: usize>(rimpy_config: Arc<RimPyConfig>, mods_config: Arc<RwLock<ModsConfig>>, mods: ModList, hint_tx: SyncSender<String>) -> Self {
        let selected = arc_mutex_none::<String>();
        let (tx, rx) = sync_channel(SIZE);

        let active_mods = mods_config.read().map_or_else(|_| Vec::new(), |mc| mc.activeMods.clone());
        let inactive_pids = mods.package_ids().map(|pids| pids.into_iter().filter(|pid| !active_mods.contains(pid)).collect())
            .unwrap_or_default();

        let active = ModListing::new(active_mods, &mods.mods, &selected, Some(String::from("Active")), tx.clone());
        let inactive = ModListing::new(inactive_pids, &mods.mods, &selected, Some(String::from("Inactive")), tx);

        let mod_info_widget = ModInfo::new(mods.mods.clone(), selected);

        let btns = ButtonsContainer::generate(hint_tx);

        Self {
            mods,
            inactive,
            active,
            mod_info_widget,
            btns,
            rimpy_config,
            mods_config,
            rx,
        }
    }

    fn tick(&mut self) {
        loop {
            match self.rx.try_recv() {
                Ok(msg) => { msg.run(&mut self.inactive.items, &mut self.active.items).log_if_err(); },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("mods panel mpsc unexpectedly disconnected"),
            }
        }
    }

    fn render(&mut self, ui: &mut Ui) -> Response {
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
                row.col(|ui| {ui.add(&mut self.mod_info_widget);});
                row.col(|ui| {ui.add(&self.inactive);});
                row.col(|ui| {ui.add(&self.active);});
                row.col(|ui| {ui.add(&self.btns);});
            }));
        });

        scope.response
    }
}

impl Widget for &mut ModsPanel<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.tick();
        self.render(ui)
    }
}

