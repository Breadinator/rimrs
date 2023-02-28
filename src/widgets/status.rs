use crate::{
    mods::ModListValidationResult, traits::LogIfErr, validate, widgets::ModListing, ModMetaData,
};
use eframe::egui::{Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
    time::SystemTime,
};

/// How often should it re-validate.
///
/// I wanted it to be `u16` but `Duration::as_millis` returns a u128.
/// TODO: In future, it'd be better to use an observer pattern of some sort to know when to update.
pub const UPDATE_FREQUENCY_MS: u128 = 200;

#[derive(Debug)]
pub struct Status<'a> {
    active_mods: Rc<RefCell<ModListing<'a>>>,
    mmd: Arc<Mutex<HashMap<String, ModMetaData>>>,
    latest: Option<ModListValidationResult>,
    last_updated: SystemTime,
}

impl<'a> Status<'a> {
    #[must_use]
    pub fn new(
        active_mods: Rc<RefCell<ModListing<'a>>>,
        mmd: Arc<Mutex<HashMap<String, ModMetaData>>>,
    ) -> Self {
        Self {
            active_mods,
            mmd,
            latest: None,
            last_updated: SystemTime::UNIX_EPOCH,
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
        self.last_updated = SystemTime::now();
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
        if let Some(elapsed) = self.last_updated.elapsed().log_if_err() {
            if elapsed.as_millis() > UPDATE_FREQUENCY_MS {
                self.update();
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
