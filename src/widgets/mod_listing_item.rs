use crate::{
    ModMetaData,
    traits::{
        LogIfErr,
        TableRower,
        LockIgnorePoisoned,
    },
    helpers::vec_ops::MultiVecOp,
    glyphs,
};
use std::{
    sync::{
        Arc,
        Mutex,
        mpsc::Sender,
    },
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};
use eframe::egui::{
    Widget,
    SelectableLabel,
    Ui,
    Response,
};
use egui_extras::TableRow;

/// A single mod. Shows its display name and buttons to reorder it.
///
/// Todo: add visual buttons to reorder items
#[derive(Debug, Clone)]
pub struct ModListingItem<'a> {
    pub package_id: String,
    pub mod_meta_data: Option<Arc<Mutex<HashMap<String, ModMetaData>>>>,
    pub selected: Rc<RefCell<Option<String>>>,
    tx: Sender<MultiVecOp<'a, ModListingItem<'a>>>,
}

impl<'a> ModListingItem<'a> {
    #[must_use]
    pub fn new(
        package_id: String,
        mod_meta_data: Arc<Mutex<HashMap<String, ModMetaData>>>,
        selected: Rc<RefCell<Option<String>>>,
        tx: Sender<MultiVecOp<'a, ModListingItem<'a>>>,
    ) -> Self {
        Self {
            package_id,
            mod_meta_data: Some(mod_meta_data),
            selected,
            tx,
        }
    }

    fn get_display_name(&self) -> String {
        self.mod_meta_data.as_ref()
            .map(|mmd| mmd.lock_ignore_poisoned()).as_ref()
            .and_then(|mmd| mmd.get(&self.package_id))
            .and_then(|m| m.name.clone())
            .unwrap_or_else(|| self.package_id.clone())
    }

    fn pid_matches_predicate(pid: String) -> Box<dyn for<'b> Fn(&'b ModListingItem<'_>) -> bool + 'a> {
        Box::new(move |item: &ModListingItem| item.package_id == pid)
    }

    fn toggle_activated(&self) {
        let pid = self.package_id.clone();
        self.tx.send(MultiVecOp::Swap(ModListingItem::pid_matches_predicate(pid)))
            .log_if_err();
    }

    fn move_up(&self) {
        let pid = self.package_id.clone();
        self.tx.send(MultiVecOp::MoveUp(Box::new(move |item| item.package_id == pid)))
            .log_if_err();
    }

    fn move_down(&self) {
        let pid = self.package_id.clone();
        self.tx.send(MultiVecOp::MoveDown(Box::new(move |item| item.package_id == pid)))
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
            let up = ui.selectable_label(false, glyphs::ARROW_UP);
            if up.clicked() {
                self.move_up();
            }
        });

        row.col(|ui| {
            let down = ui.selectable_label(false, glyphs::ARROW_DOWN);
            if down.clicked() {
                self.move_down();
            }
        });

        row.col(|ui| {
            let mut sel = self.selected.borrow_mut();
            let is_selected = sel.clone().map_or(false, |pid| self.package_id == pid);
            let lab = ui.add(SelectableLabel::new(is_selected, self.get_display_name()));

            if lab.clicked() {
                *sel = Some(self.package_id.clone().to_lowercase());
            }

            if lab.double_clicked() {
                self.toggle_activated();
            }

            // if lab.middle_clicked() {
            //     self.move_up();
            // }

            // if lab.secondary_clicked() {
            //     self.move_down();
            // }
        });
    }
}

