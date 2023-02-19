use crate::{mods::ModListValidationResult, traits::LogIfErr, validator_thread};
use eframe::egui::{Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use std::sync::mpsc::{self, Receiver, SyncSender};

#[derive(Debug)]
pub struct Status {
    latest: Option<ModListValidationResult>,
    rx: Receiver<ModListValidationResult>,
    resp_tx: SyncSender<ModListValidationResult>,
    tx: SyncSender<validator_thread::Message>,
}

impl Status {
    #[must_use]
    pub fn new(tx: SyncSender<validator_thread::Message>) -> Self {
        let (resp_tx, rx) = mpsc::sync_channel::<ModListValidationResult>(3);

        Self {
            latest: None,
            rx,
            resp_tx,
            tx,
        }
    }

    fn update(&mut self) {
        // send req
        let package_ids: Vec<String> = Vec::new(); // todo
        let msg = validator_thread::Message::Validate(package_ids, self.resp_tx.clone());
        self.tx.try_send(msg).log_if_err();

        // recv resp
        while let Ok(res) = self.rx.try_recv() {
            self.latest = Some(res);
        }
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

impl Widget for &mut Status {
    fn ui(self, ui: &mut Ui) -> Response {
        self.update();

        ui.scope(|ui| {
            if let Some(res) = self.latest.as_ref() {
                Status::display(ui, res);
            }
        })
        .response
    }
}
