use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use crate::ModMetaData;

#[derive(Debug, Default, Clone)]
pub struct ModInfoWidget(Option<ModMetaData>);

impl From<ModMetaData> for ModInfoWidget {
    fn from(mmd: ModMetaData) -> Self {
        Self(Some(mmd))
    }
}

impl Widget for &ModInfoWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label("mod info widget")
    }
}

