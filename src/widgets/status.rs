use crate::{mods::ModListValidationResult, validate, widgets::ModListing, ModMetaData};
use eframe::egui::{Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct Status<'a> {
    active_mods: Rc<RefCell<ModListing<'a>>>,
    mmd: Arc<Mutex<HashMap<String, ModMetaData>>>,
    latest: Option<ModListValidationResult>,
    should_update: *mut bool,
}

impl<'a> Status<'a> {
    #[must_use]
    pub fn new(
        active_mods: Rc<RefCell<ModListing<'a>>>,
        mmd: Arc<Mutex<HashMap<String, ModMetaData>>>,
        should_update: *mut bool,
    ) -> Self {
        Self {
            active_mods,
            mmd,
            latest: None,
            should_update,
        }
    }

    fn update(&mut self) {
        let mods: Vec<_> = self
            .active_mods
            .borrow()
            .items
            .iter()
            .map(|item| item.package_id.clone())
            .collect();
        self.latest = Some(validate(&self.mmd, &mods));
    }

    fn display(ui: &mut Ui, res: &ModListValidationResult) {
        let w = ui.available_width() / 2.0;
        let h = 16.0;

        TableBuilder::new(ui)
            .column(Column::exact(w))
            .column(Column::remainder())
            .body(|mut body| {
                body.row(h, |mut row| {
                    row.col(|ui| {
                        ui.label(format!("Warnings: {}", res.warnings().map_or(0, Vec::len)));
                    });
                    row.col(|ui| {
                        ui.label(format!("Errors: {}", res.errors().map_or(0, Vec::len)));
                    });
                });
            });
    }
}

impl Widget for &mut Status<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        // SAFETY: only using this bool on the main thread (on ModsPanel)
        unsafe {
            if *self.should_update {
                self.update();
                *self.should_update = false;
            }
        }

        ui.scope(|ui| {
            if let Some(res) = self.latest.as_ref() {
                Status::display(ui, res);
            }
        })
        .response
    }
}
