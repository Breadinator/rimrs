use rimrs::*;
use eframe::{
    NativeOptions,
    run_native
};

fn main() {
    env_logger::init();

    let opts = NativeOptions {
        // min_window_size: Some(egui::vec2(320.0, 100.0)),
        // initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    run_native("RimRs", opts, Box::new(|cc| Box::new(RimRs::new(cc))));
}

