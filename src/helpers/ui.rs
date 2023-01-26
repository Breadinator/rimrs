use eframe::egui::Ui;

pub fn force_width(ui: &mut Ui, w: f32) {
    ui.set_width(w);
    ui.set_min_width(w);
    ui.set_max_width(w);
}
