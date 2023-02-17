use crate::helpers::traits::LogIfErr;
use eframe::egui::{Response, Ui, Widget};
use std::path::PathBuf;

/// Wrapper for a path.
/// Basically just shows a [`eframe::egui::Button`] that opens the path when clicked.
#[derive(Debug, Clone)]
pub struct PathLabel(PathBuf);

impl PathLabel {
    #[must_use]
    pub fn new(mut path: PathBuf) -> Self {
        if path.file_name() == Some(std::ffi::OsStr::new("About.xml")) {
            path.pop(); // now ends in /About
            path.pop(); // should now be the root mod dir
        }
        Self(path)
    }
}

impl Widget for &PathLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let path = self.0.as_os_str().to_str().unwrap_or_default();
        let lab = ui.button(path); // kinda ugly but at least it works lol

        if lab.clicked() && !path.is_empty() {
            open::that(self.0.clone()).log_if_err();
        }

        lab
    }
}
