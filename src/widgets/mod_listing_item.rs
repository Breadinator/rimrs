use crate::{
    ModMetaData,
    traits::{
        LogIfErr,
        TableRower,
    },
    helpers::vec_ops::MultiVecOp,
};
use std::{
    sync::{
        Arc,
        Mutex,
        mpsc::SyncSender,
    },
    collections::HashMap,
};
use eframe::egui::{
    Widget,
    SelectableLabel,
    Ui,
    Response,
};
use egui_extras::TableRow;

/// Todo: add visual buttons to reorder items
#[derive(Debug, Clone)]
pub struct ModListingItem<'a> {
    pub package_id: String,
    pub mod_meta_data: Option<Arc<Mutex<HashMap<String, ModMetaData>>>>,
    pub selected: Arc<Mutex<Option<String>>>,
    tx: SyncSender<MultiVecOp<'a, ModListingItem<'a>>>,
}

impl<'a> ModListingItem<'a> {
    #[must_use]
    pub fn new(
        package_id: String,
        mod_meta_data: Arc<Mutex<HashMap<String, ModMetaData>>>,
        selected: Arc<Mutex<Option<String>>>,
        tx: SyncSender<MultiVecOp<'a, ModListingItem<'a>>>,
    ) -> Self {
        Self {
            package_id,
            mod_meta_data: Some(mod_meta_data),
            selected,
            tx,
        }
    }

    fn get_display_name(&self) -> String {
        if let Some(mmd_mutex) = &self.mod_meta_data {
            if let Ok(mmd) = mmd_mutex.lock() {
                if let Some(package_mmd) = mmd.get(&self.package_id) {
                    if let Some(name) = package_mmd.name.clone() {
                        return name;
                    }
                }
            }
        }

        self.package_id.clone()
    }

    fn pid_matches_predicate(pid: String) -> Box<dyn for<'b> Fn(&'b ModListingItem<'_>) -> bool + 'a> {
        Box::new(move |item: &ModListingItem| item.package_id == pid)
    }

    fn toggle_activated(&self) {
        let pid = self.package_id.clone();
        self.tx.try_send(MultiVecOp::Swap(ModListingItem::pid_matches_predicate(pid)))
            .log_if_err();
    }

    fn move_up(&self) {
        let pid = self.package_id.clone();
        self.tx.try_send(MultiVecOp::MoveUp(Box::new(move |item| item.package_id == pid)))
            .log_if_err();
    }

    fn move_down(&self) {
        let pid = self.package_id.clone();
        self.tx.try_send(MultiVecOp::MoveDown(Box::new(move |item| item.package_id == pid)))
            .log_if_err();
    }
}

impl<'a> Widget for &ModListingItem<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        const BUTTON_WIDTH: f32 = 16.0;
        const ROW_HEIGHT: f32 = 16.0;

        ui.push_id(&self.package_id, |ui| {
            egui_extras::TableBuilder::new(ui)
                .column(egui_extras::Column::exact(BUTTON_WIDTH))
                .column(egui_extras::Column::exact(BUTTON_WIDTH))
                .column(egui_extras::Column::remainder())
                .body(|mut body| body.row(ROW_HEIGHT, |r| self.table_row(r)));
        }).response
    }
}

impl TableRower for &ModListingItem<'_> {
    fn table_row(self, mut row: TableRow) {
        row.col(|ui| {
            let up = ui.selectable_label(false, "U");
            if up.clicked() {
                self.move_up();
            }
        });

        row.col(|ui| {
            let down = ui.selectable_label(false, "D");
            if down.clicked() {
                self.move_down();
            }
        });

        row.col(|ui| {
            let lab = if let Ok(mut sel) = self.selected.try_lock() {
                let is_selected = (*sel).clone().map_or(false, |sel| self.package_id == sel);
                let l = ui.add(SelectableLabel::new(is_selected, self.get_display_name()));
                if l.clicked() {
                    *sel = Some(self.package_id.clone().to_lowercase());
                }
                l
            } else {
                ui.selectable_label(false, self.get_display_name())
            };

            if lab.double_clicked() {
                self.toggle_activated();
            }

            // placeholder button
            if lab.middle_clicked() {
                self.move_up();
            }

            // placeholder button
            if lab.secondary_clicked() {
                self.move_down();
            }
        });
    }
}

