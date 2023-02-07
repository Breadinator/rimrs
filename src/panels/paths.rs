use crate::{
    helpers::traits::{LogIfErr, ToStringOrEmpty},
    serialization::rimpy_config::RimPyConfig,
};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        mpsc::SyncSender,
    },
};
use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use egui_extras::{
    TableBuilder,
    Column,
    TableRow,
};

#[derive(Debug, Clone)]
pub struct PathsPanel {
    rimpy_config: Arc<RimPyConfig>,
    version: String,
    hint_tx: SyncSender<String>,
}

impl PathsPanel {
    #[must_use]
    pub fn new(rimpy_config: Arc<RimPyConfig>, version: String, hint_tx: SyncSender<String>) -> Self {
        Self {
            rimpy_config,
            version,
            hint_tx,
        }
    }

    fn build_table(ui: &mut Ui, conf: &Arc<RimPyConfig>, version: &String, hint_tx: &SyncSender<String>) -> Response {
        ui.scope(|ui| {
            TableBuilder::new(ui)
                .column(Column::auto())
                .column(Column::remainder())
                .column(Column::auto())
                .body(|mut body| {
                    const H: f32 = 24.0;

                    macro_rules! r {
                        ($func:ident) => {
                            body.row(H, |mut row| $func(&mut row, conf, hint_tx))
                        }
                    }

                    body.row(H, |mut row| row_1(&mut row, version, hint_tx));
                    r!(row_2);
                    r!(row_3);
                    r!(row_4);
                    r!(row_5);
                });
        }).response
    }
}

impl Widget for &mut PathsPanel {
    fn ui(self, ui: &mut Ui) -> Response {
        PathsPanel::build_table(ui, &self.rimpy_config, &self.version, &self.hint_tx)
    }
}

fn row_1(row: &mut TableRow, version: &String, hint_tx: &SyncSender<String>) {
    row.col(|ui| {
        open_rimpy_button(ui, hint_tx);
    });
    row.col(|ui| {
        ui.label(format!("Game version: {version}"));
    });
}

fn row_2(row: &mut TableRow, conf: &Arc<RimPyConfig>, hint_tx: &SyncSender<String>) {
    row.col(|ui| {
        open_button(ui, "Game folder", &conf.folders.game_folder, Some("Open folder containing RimWorld"), hint_tx);
    });
    row.col(|ui| {
        ui.label(conf.folders.game_folder.to_string_or_empty());
    });
}

fn row_3(row: &mut TableRow, conf: &Arc<RimPyConfig>, hint_tx: &SyncSender<String>) {
    row.col(|ui| {
        open_button(ui, "Config folder", &conf.folders.config_folder, Some("Open the RimWorld game config folder"), hint_tx);
    });
    row.col(|ui| {
        ui.label(conf.folders.config_folder.to_string_or_empty());
    });
}

fn row_4(row: &mut TableRow, conf: &Arc<RimPyConfig>, hint_tx: &SyncSender<String>) {
     row.col(|ui| {
        open_button(ui, "Steam mods", &conf.folders.steam_mods, Some("Open folder where steam mods are stored"), hint_tx);
    });
    row.col(|ui| {
        ui.label(conf.folders.steam_mods.to_string_or_empty());
    });
}

fn row_5(row: &mut TableRow, conf: &Arc<RimPyConfig>, hint_tx: &SyncSender<String>) {
    row.col(|ui| {
        open_button(ui, "Local mods", &conf.folders.local_mods, Some("Open folder where local mods are stored (Mods folder)"), hint_tx);
    });
    row.col(|ui| {
        ui.label(conf.folders.local_mods.to_string_or_empty());
    });
}

pub fn open_rimpy_button(ui: &mut Ui, hint_tx: &SyncSender<String>) {
    let settings_btn = ui.button("Settings");
    if settings_btn.clicked() {
        crate::helpers::config::get_config_dir().map(open::that)
            .log_if_err();
    }
    if settings_btn.hovered {
        hint_tx.try_send(String::from("Open folder where RimPy stores its config files")).ok();
    }
}

pub fn open_button(ui: &mut Ui, lab: &str, path: &Option<PathBuf>, hint: Option<&str>, hint_tx: &SyncSender<String>) -> Response {
    let mut btn = ui.button(lab);
    if let Some(path) = path {
        if btn.clicked() {
            open::that(path)
                .log_if_err();
        }

        if btn.hovered() {
            if let Some(hint) = hint {
                hint_tx.try_send(String::from(hint)).ok();
            }
        }
    } else {
        btn.enabled = false;
    }
    btn
}

