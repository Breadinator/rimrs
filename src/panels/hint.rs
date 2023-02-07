use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use std::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct HintPanel {
    pub hint: Option<String>,
    rx: Receiver<String>,
}

impl HintPanel {
    #[must_use]
    pub fn new(rx: Receiver<String>) -> Self {
        Self {
            hint: None,
            rx,
        }
    }

    pub fn update(&mut self) {
        while let Ok(hint) = self.rx.try_recv() {
            self.hint = Some(hint);
        }
    }
}

impl Widget for &mut HintPanel {
    fn ui(self, ui: &mut Ui) -> Response {
        self.update();
        ui.label(&self.hint.clone().unwrap_or_default())
    }
}

