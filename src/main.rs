use eframe::{run_native, NativeOptions};
use rimrs::*;
use std::sync::mpsc::sync_channel;

fn main() {
    #[cfg(feature = "dotenv")]
    {
        dotenv::dotenv().ok();
    }
    env_logger::init();

    let (writer_tx, writer_rx) = sync_channel(3);
    let writer_handle = writer_thread::spawn(writer_rx);

    let opts = NativeOptions {
        // min_window_size: Some(egui::vec2(320.0, 100.0)),
        // initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    let writer_tx_for_app = writer_tx.clone();
    run_native(
        "RimRs",
        opts,
        Box::new(move |cc| Box::new(RimRs::new(cc, writer_tx_for_app))),
    );

    writer_tx.send(writer_thread::Message::Stop).unwrap();
    writer_handle.join().unwrap();
}
