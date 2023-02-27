use eframe::egui::{self, Response, Ui, Widget};
use std::sync::mpsc::{SyncSender, TrySendError};

/// The buttons that appear to the right of the mod lists.
pub struct Button<'a> {
    label: &'a str,
    action: Option<Box<dyn Fn() + 'a>>,
    is_enabled_fn: Option<Box<dyn Fn() -> bool + 'a>>,
    hint_sender: Option<HintSender<'a>>,
}

impl std::fmt::Debug for Button<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Button {{ label: {:?}, hint: {:?} }}",
            self.label,
            self.hint_sender.as_ref().map(|h| h.msg)
        ))
    }
}

impl<'a> Button<'a> {
    #[must_use]
    pub fn builder(label: &'a str) -> ButtonBuilder<'a> {
        ButtonBuilder::new(label)
    }

    /// Checks if the [`Button`] should be enabled, using the function stored in `is_enabled_fn`.
    /// Returns `true` if `None`.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.is_enabled_fn.as_ref().map_or(true, |f| f())
    }
}

impl<'a> Widget for &Button<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let btn = egui::Button::new(self.label);
        let resp = ui.add_enabled(self.is_enabled(), btn);

        if resp.clicked() {
            if let Some(action) = self.action.as_ref() {
                (action)();
            }
        }

        // Doesn't trigger hover when disabled; might have to implement own hover logic if no given workaround?
        if resp.hovered() {
            self.hint_sender.as_ref().map(HintSender::try_send);
        }

        resp
    }
}

/// Used to build [`Button`]s.
#[allow(clippy::module_name_repetitions)]
pub struct ButtonBuilder<'a> {
    label: &'a str,
    action: Option<Box<dyn Fn() + 'a>>,
    is_enabled_fn: Option<Box<dyn Fn() -> bool + 'a>>,
    hint_sender: Option<HintSender<'a>>,
}

impl<'a> ButtonBuilder<'a> {
    #[must_use]
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            action: None,
            is_enabled_fn: None,
            hint_sender: None,
        }
    }

    #[must_use]
    pub fn build(self) -> Button<'a> {
        Button::from(self)
    }

    #[must_use]
    pub fn action(mut self, action: Box<dyn Fn() + 'a>) -> Self {
        self.action = Some(action);
        self
    }

    #[must_use]
    pub fn hint(mut self, hint: &'a str, hint_tx: SyncSender<String>) -> Self {
        self.hint_sender = Some(HintSender::new(hint, hint_tx));
        self
    }

    #[must_use]
    pub fn is_enabled_fn(mut self, is_enabled_fn: Box<dyn Fn() -> bool + 'a>) -> Self {
        self.is_enabled_fn = Some(is_enabled_fn);
        self
    }
}

impl<'a> From<ButtonBuilder<'a>> for Button<'a> {
    fn from(builder: ButtonBuilder<'a>) -> Self {
        Button {
            label: builder.label,
            action: builder.action,
            hint_sender: builder.hint_sender,
            is_enabled_fn: builder.is_enabled_fn,
        }
    }
}

/// Couples the static message to send as a hint and the `tx` to send it by.
struct HintSender<'a> {
    /// An owned [`String`] will be generated from this.
    msg: &'a str,
    tx: SyncSender<String>,
}

impl<'a> HintSender<'a> {
    #[must_use]
    pub fn new(msg: &'a str, tx: SyncSender<String>) -> Self {
        Self { msg, tx }
    }

    pub fn try_send(&self) {
        match self.tx.try_send(String::from(self.msg)) {
            Ok(_) => {}
            Err(TrySendError::Full(_)) => {
                log::warn!("Hint channel full");
            }
            Err(TrySendError::Disconnected(_)) => {
                log::error!("Hint mpsc channel unexpectedly disconnected");
            }
        }
    }
}
