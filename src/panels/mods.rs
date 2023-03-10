use crate::{
    helpers::vec_ops::MultiVecOp,
    widgets::{ButtonsContainer, ModInfo, ModListing, ModListingItem, Status, STATUS_HEIGHT},
    writer_thread, ModList, ModsConfig, RimPyConfig,
};
use eframe::egui::{Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    sync::{
        mpsc::{channel, Receiver, Sender, SyncSender, TryRecvError},
        Arc,
    },
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
    status: Status<'a>,
    should_update_status: *mut bool,
}

impl ModsPanel<'_> {
    /// Makes a new mods panel
    #[must_use]
    #[allow(clippy::too_many_arguments)] // stay mad
    pub fn new(
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

        let (active, inactive) = ModListing::new_pair(
            mods_config.activeMods.clone(),
            &mods,
            &selected,
            &direct_vecop_tx,
        );
        let active = Rc::new(RefCell::new(active));

        let mod_info_widget = ModInfo::new(mods.mods.clone(), selected.clone());

        let btns = ButtonsContainer::generate(
            hint_tx,
            writer_thread_tx,
            change_mod_list_tx.clone(),
            active.clone(),
            mods.mods.clone(),
            exe_path,
            args,
        );

        let should_update_status = Box::into_raw(Box::from(true));
        let status = Status::new(active.clone(), mods.mods.clone(), should_update_status);

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
            status,
            should_update_status,
        }
    }

    fn tick(&mut self) {
        self.run_vecops();
        self.change_mod_lists();
    }

    /// Used to update various crate-wide state.
    fn on_active_modlist_change(&mut self) {
        crate::CHANGED_ACTIVE_MODS.set();

        // SAFETY: only using this bool on the main thread
        unsafe {
            *self.should_update_status = true;
        }
    }

    fn run_vecops(&mut self) {
        let mut active_guard = self.active.borrow_mut();
        let mut changed = false;

        loop {
            let res = self.direct_vecop_rx.try_recv().map(|msg| {
                msg.run(
                    (&mut self.inactive.items).into(),
                    (&mut active_guard.items).into(),
                )
            });
            match res {
                Ok(Ok(_)) => changed = true,
                Ok(Err(err)) => log::error!("{err:?}"),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    panic!("mods panel mpsc channel unexpectedly disconnected")
                }
            }
        }

        drop(active_guard);

        if changed {
            self.on_active_modlist_change();
        }
    }

    fn change_mod_lists(&mut self) {
        let mut active_guard = self.active.borrow_mut();
        let mut changed = false;

        loop {
            match self.change_mod_list_rx.try_recv() {
                Ok(new_mod_list) => {
                    let (active, inactive) = ModListing::new_pair(
                        new_mod_list,
                        &self.mods,
                        &self.selected,
                        &self.direct_vecop_tx,
                    );
                    *active_guard = active;
                    self.inactive = inactive;
                    changed = true;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    panic!("mods panel mpsc channel unexpectedly disconnected")
                }
            }
        }

        drop(active_guard);

        if changed {
            self.on_active_modlist_change();
        }
    }

    fn render(&mut self, ui: &mut Ui) -> Response {
        let scope = ui.scope(|ui| {
            let w = ui.available_width() / 10.0;
            let mod_info_width = 4.0 * w;
            let mod_listing_width = 2.5 * w;

            let h = ui.available_height() - STATUS_HEIGHT;

            let active_cloned = self.active.borrow().clone();

            TableBuilder::new(ui)
                .column(Column::exact(mod_info_width))
                .column(Column::exact(mod_listing_width))
                .column(Column::exact(mod_listing_width))
                .column(Column::remainder())
                .body(|mut body| {
                    body.row(h, |mut row| {
                        row.col(|ui| {
                            ui.add(&mut self.mod_info_widget);
                        });
                        row.col(|ui| {
                            ui.add(&self.inactive);
                        });
                        row.col(|ui| {
                            ui.add(&active_cloned);
                        });
                        row.col(|ui| {
                            ui.add(&self.btns);
                        });
                    });
                });

            ui.add(&mut self.status);
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

impl Drop for ModsPanel<'_> {
    fn drop(&mut self) {
        // SAFETY: safe to drop because the only other holder of this pointer
        // will also be dropped (the Status widget)
        unsafe {
            drop(Box::from_raw(self.should_update_status));
        }
    }
}
