use eframe::egui::{
    Widget,
    Ui,
    Response,
};
use std::{
    sync::mpsc::Receiver,
    time::{
        Instant,
        Duration,
    },
    collections::VecDeque,
};

const UPDATE_DURATION: Duration = Duration::from_millis(500); // 0.5s

#[derive(Debug)]
pub struct HintPanel {
    pub hint: Option<String>,
    rx: Receiver<String>,
    last_updated: Option<Instant>,
    buf: VecDeque<String>,
}

impl HintPanel {
    #[must_use]
    pub fn new(rx: Receiver<String>) -> Self {
        Self {
            hint: None,
            rx,
            last_updated: None,
            buf: VecDeque::new(),
        }
    }

    pub fn update(&mut self) {
        while let Ok(hint) = self.rx.try_recv() {
            self.buf.push_back(hint);
            // self.hint = Some(hint);
        }

        if self.buf.is_empty() {
            return;
        }

        if let Some(last_updated) = self.last_updated.as_ref() {
            if last_updated.elapsed() > UPDATE_DURATION {
                self.hint = self.buf.pop_front();
                self.last_updated = Some(Instant::now());
            }
        } else {
            self.hint = self.buf.pop_front();
            self.last_updated = Some(Instant::now());
        }
    }
}

impl Widget for &mut HintPanel {
    fn ui(self, ui: &mut Ui) -> Response {
        self.update();
        ui.label(&self.hint.clone().unwrap_or_default())
    }
}

