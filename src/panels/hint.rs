use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use std::{
    sync::mpsc::Receiver,
    time::{
        SystemTime,
        Duration,
    },
};

const UPDATE_DURATION: Duration = Duration::from_millis(500); // 0.5s

#[derive(Debug)]
pub struct HintPanel {
    pub hint: Option<String>,
    rx: Receiver<String>,
    last_updated: Option<SystemTime>,
    buf: Option<String>,
}

impl HintPanel {
    #[must_use]
    pub fn new(rx: Receiver<String>) -> Self {
        Self {
            hint: None,
            rx,
            last_updated: None,
            buf: None,
        }
    }

    pub fn update(&mut self) {
        while let Ok(hint) = self.rx.try_recv() {
            self.buf = Some(hint);
        }

        if self.buf.is_none() || self.buf == self.hint {
            return;
        }

        if let Some(last_updated) = self.last_updated.as_mut() {
            log::debug!("{:?}", last_updated.elapsed());
            if last_updated.elapsed().unwrap_or(UPDATE_DURATION) >= UPDATE_DURATION {
                self.hint = self.buf.take();
                self.buf = None;
                *last_updated = SystemTime::now();
            }
        } else {
            self.hint = self.buf.take();
            self.buf = None;
            self.last_updated = Some(SystemTime::now());
        }
    }
}

impl Widget for &mut HintPanel {
    fn ui(self, ui: &mut Ui) -> Response {
        self.update();
        ui.label(&self.hint.clone().unwrap_or_default())
    }
}

