use std::path::PathBuf;
use crate::helpers::traits::LogIfErr;
use eframe::egui::{
    Widget,
    Ui,
    Response,
};

#[derive(Debug, Clone)]
pub struct PathLabel(pub PathBuf);

impl Widget for &PathLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let path = self.0.as_os_str().to_str().unwrap_or_default();
        let lab = ui.label(path);

        // idk why this isn't working?
        // lab.clicked() is false even when clicked
        if lab.clicked() && !path.is_empty() {
            open::that(self.0.clone())
                .log_if_err();
        }

        lab
    }
}
