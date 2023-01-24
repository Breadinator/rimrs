use crate::{
    helpers::traits::*,
    serialization::{
        RimPyConfig,
        ReadRimPyConfigError,
    },
};
use std::path::PathBuf;
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

#[derive(Debug, Clone, Default)]
pub struct PathsPanel {
    rimpy_config: Option<RimPyConfig>,
}

impl PathsPanel {
    fn read_rimpy_config_if_uncached(&mut self) -> Result<(), ReadRimPyConfigError> {
        if self.rimpy_config.is_some() {
            return Ok(());
        }
        self.rimpy_config = Some(RimPyConfig::from_file()?);
        Ok(())
    }

    fn build_table(ui: &mut Ui, conf: &mut RimPyConfig) -> Response {
        ui.group(|ui| {
            TableBuilder::new(ui)
                .column(Column::auto())
                .column(Column::remainder())
                .column(Column::auto())
                .body(|mut body| {
                    macro_rules! r {
                        ($func:ident) => {
                            body.row(24.0, |mut row| $func(&mut row, conf))
                        }
                    }

                    r!(row_1);
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
        if let Err(e) = self.read_rimpy_config_if_uncached() {
            return ui.label(format!("An error occurred: {e:?}"));
        }

        match self.rimpy_config.as_mut() {
            Some(rimpy_config) => PathsPanel::build_table(ui, rimpy_config),
            None => {
                ui.group(|ui| {
                    ui.label("An error occurred when reading the RimPy config file!");
                }).response
            },
        }
    }
}

fn row_1(row: &mut TableRow, _conf: &mut RimPyConfig) {
    row.col(|ui| {
        open_rimpy_button(ui);
    });
    row.col(|ui| {
        ui.label("Game version: ???");
    });
}

fn row_2(row: &mut TableRow, conf: &mut RimPyConfig) {
    row.col(|ui| {
        open_button(ui, "Game folder", &conf.folders.game_folder);
    });
    row.col(|ui| {
        ui.label(conf.folders.game_folder.to_string_or_empty());
    });
}

fn row_3(row: &mut TableRow, conf: &mut RimPyConfig) {
    row.col(|ui| {
        open_button(ui, "Config folder", &conf.folders.config_folder);
    });
    row.col(|ui| {
        ui.label(conf.folders.config_folder.to_string_or_empty());
    });
}

fn row_4(row: &mut TableRow, conf: &mut RimPyConfig) {
     row.col(|ui| {
        open_button(ui, "Steam mods", &conf.folders.steam_mods);
    });
    row.col(|ui| {
        ui.label(conf.folders.steam_mods.to_string_or_empty());
    });
}

fn row_5(row: &mut TableRow, conf: &mut RimPyConfig) {
    row.col(|ui| {
        open_button(ui, "Local mods", &conf.folders.local_mods);
    });
    row.col(|ui| {
        ui.label(conf.folders.local_mods.to_string_or_empty());
    });
}

pub fn open_rimpy_button(ui: &mut Ui) {
    let settings_btn = ui.button("Settings");
    if settings_btn.clicked() {
        crate::helpers::config::get_config_dir().map(open::that)
            .log_if_err();
    }
}

pub fn open_button(ui: &mut Ui, lab: &str, path: &Option<PathBuf>) -> Response {
    let mut btn = ui.button(lab);
    if let Some(path) = path {
        if btn.clicked() {
            open::that(path)
                .log_if_err();
        }
    } else {
        btn.enabled = false;
    }
    btn
}

