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
    queued: Option<String>,
}

impl HintPanel {
    #[must_use]
    pub fn new(rx: Receiver<String>) -> Self {
        Self {
            hint: None,
            rx,
            last_updated: None,
            queued: None,
        }
    }

    pub fn update(&mut self) {
        while let Ok(hint) = self.rx.try_recv() {
            self.queued = Some(hint);
        }

        if self.queued.is_none() || self.queued == self.hint {
            return;
        }

        if let Some(last_updated) = self.last_updated.as_mut() {
            if last_updated.elapsed().unwrap_or(UPDATE_DURATION) >= UPDATE_DURATION {
                self.hint = self.queued.take();
                self.queued = None;
                *last_updated = SystemTime::now();
            }
        } else {
            self.hint = self.queued.take();
            self.queued = None;
            self.last_updated = Some(SystemTime::now());
        }
    }
}

impl Widget for &mut HintPanel {
    fn ui(self, ui: &mut Ui) -> Response {
        self.update();

        if let Some(hint) = self.hint.as_ref() {
            ui.label(hint)
        } else {
            ui.label("")
        }
    }
}

