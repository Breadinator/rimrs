use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use super::Button;

#[derive(Debug)]
pub struct Btns<'a>(Vec<Button<'a>>);

impl Default for Btns<'_> {
    fn default() -> Self {
        Self(vec![
             Button::clear(),
             Button::sort(),
             Button::save(),
             Button::run(),
        ])
    }
}

impl<'a> Widget for &'a Btns<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.scope(|ui| {
            for btn in &self.0 {
                ui.add(btn);
            }
        }).response
    }
}

