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
    helpers::vec_ops::MultiVecOp,
    writer_thread,
};
use std::{
    sync::{
        Arc,
        mpsc::{
            channel,
            Sender,
            SyncSender,
            Receiver,
            TryRecvError,
        },
    },
    rc::Rc,
    cell::RefCell,
    path::PathBuf,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ModsPanel<'a> {
    pub mods: ModList,
    inactive: ModListing<'a>,
    active: Rc<RefCell<ModListing<'a>>>,
    mod_info_widget: ModInfo,
    btns: ButtonsContainer<'a>,
    rimpy_config: Rc<RimPyConfig>,
    mods_config: Arc<ModsConfig>,
    direct_vecop_rx: Receiver<MultiVecOp<'a, ModListingItem<'a>>>,
    direct_vecop_tx: Sender<MultiVecOp<'a, ModListingItem<'a>>>,
    change_mod_list_rx: Receiver<Vec<String>>,
    change_mod_list_tx: Sender<Vec<String>>,
    selected: Rc<RefCell<Option<String>>>,
}

impl ModsPanel<'_> {
    /// Makes a new mods panel
    #[must_use]
    pub fn new<const SIZE: usize>(
        rimpy_config: Rc<RimPyConfig>,
        mods_config: Arc<ModsConfig>,
        mods: ModList,
        hint_tx: &SyncSender<String>,
        writer_thread_tx: SyncSender<writer_thread::Message>,
        exe_path: PathBuf,
        args: Option<String>,
    ) -> Self {
        let selected = Rc::new(RefCell::new(None));
        let (direct_vecop_tx, direct_vecop_rx) = channel();
        let (change_mod_list_tx, change_mod_list_rx) = channel();

        let (active, inactive) = ModListing::new_pair(mods_config.activeMods.clone(), &mods, &selected, &direct_vecop_tx);
        let active = Rc::new(RefCell::new(active));

        let mod_info_widget = ModInfo::new(mods.mods.clone(), selected.clone());

        let btns = ButtonsContainer::generate(
            hint_tx,
            writer_thread_tx,
            change_mod_list_tx.clone(),
            active.clone(),
            exe_path,
            args,
        );

        Self {
            mods,
            inactive,
            active,
            mod_info_widget,
            btns,
            rimpy_config,
            mods_config,
            direct_vecop_rx,
            direct_vecop_tx,
            change_mod_list_rx,
            change_mod_list_tx,
            selected,
        }
    }

    fn tick(&mut self) {
        self.run_vecops();
        self.change_mod_lists();
    }

    fn run_vecops(&mut self) {
        let mut active_guard = self.active.borrow_mut();
        loop {
            let res = self.direct_vecop_rx.try_recv()
                .map(|msg| msg.run((&mut self.inactive.items).into(), (&mut active_guard.items).into()));
            match res {
                Ok(Ok(_)) => crate::CHANGED_ACTIVE_MODS.set(),
                Ok(Err(err)) => log::error!("{err:?}"),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("mods panel mpsc channel unexpectedly disconnected"),
            }
        }
    }

    fn change_mod_lists(&mut self) {
        let mut active_guard = self.active.borrow_mut();
        loop {
            match self.change_mod_list_rx.try_recv() {
                Ok(new_mod_list) => {
                    let (active, inactive) = ModListing::new_pair(new_mod_list, &self.mods, &self.selected, &self.direct_vecop_tx);
                    *active_guard = active;
                    self.inactive = inactive;
                    crate::CHANGED_ACTIVE_MODS.set();
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("mods panel mpsc channel unexpectedly disconnected"),
            }
        }
    }

    fn render(&mut self, ui: &mut Ui) -> Response {
        let scope = ui.scope(|ui| { let w = ui.available_width() / 10.0;
        let mod_info_width = 4.0 * w;
        let mod_listing_width = 2.5 * w;
        let h = ui.available_height();

        let active_cloned = self.active.borrow().clone();

        TableBuilder::new(ui)
            .column(Column::exact(mod_info_width))
            .column(Column::exact(mod_listing_width))
            .column(Column::exact(mod_listing_width))
            .column(Column::remainder())
            .body(|mut body| body.row(h, |mut row| {
                row.col(|ui| {ui.add(&mut self.mod_info_widget);});
                row.col(|ui| {ui.add(&self.inactive);});
                row.col(|ui| {ui.add(&active_cloned);});
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

