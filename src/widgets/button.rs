use eframe::egui::{
    self,
    Widget,
    Ui,
    Response,
};
use std::sync::mpsc::SyncSender;

pub struct Button<'a> {
    label: &'a str,
    action: Option<Box<dyn Fn() + 'a>>,
    hint: Option<&'a str>,
    is_enabled_fn: Option<Box<dyn Fn() -> bool + 'a>>,
    hint_tx: SyncSender<String>,
}

impl std::fmt::Debug for Button<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Button {{ label: {:?}, hint: {:?} }}", self.label, self.hint))
    }
}

impl<'a> Button<'a> {
    #[must_use]
    pub fn builder(label: &'a str, hint_tx: SyncSender<String>) -> ButtonBuilder {
        ButtonBuilder::new(label, hint_tx)
    }

    /// Checks if the button should be enabled, using the function stored in `is_enabled_fn`.
    /// Returns `true` if `None`.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.is_enabled_fn.as_ref()
            .map_or(true, |f| f())
    }

    #[must_use]
    pub fn clear(hint_tx: SyncSender<String>) -> Self {
        let action = Box::new(|| log::debug!("Unimplemented 😇")) as Box<dyn Fn() + 'a>;
        let hint = "Remove all mods, except Core and DLCs";

        Self::builder("Clear", hint_tx)
            .action(action)
            .hint(hint)
            .build()
    }

    #[must_use]
    pub fn sort(hint_tx: SyncSender<String>) -> Self {
        let action = Box::new(|| {}) as Box<dyn Fn() + 'a>;
        let hint = "Auto-sort mods";

        Self::builder("Sort", hint_tx)
            .action(action)
            .hint(hint)
            .build()
    }

    #[must_use]
    pub fn save(hint_tx: SyncSender<String>) -> Self {
        let action = Box::new(|| {}) as Box<dyn Fn() + 'a>;
        let hint = "Save the mod list to ModsConfig.xml file (applies changes to game mod list)";
        let is_enabled = Box::new(|| false) as Box<dyn Fn() -> bool + 'a>;

        Self::builder("Save", hint_tx)
            .action(action)
            .hint(hint)
            .is_enabled_fn(is_enabled)
            .build()
    }

    #[must_use]
    pub fn run(hint_tx: SyncSender<String>) -> Self {
        let action = Box::new(|| {}) as Box<dyn Fn() + 'a>;
        let hint = "Run the game";
        let is_enabled = Box::new(|| false) as Box<dyn Fn() -> bool>;

        Self::builder("Run", hint_tx)
            .action(action)
            .hint(hint)
            .is_enabled_fn(is_enabled)
            .build()
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
            if let Some(hint) = self.hint {
                self.hint_tx.try_send(String::from(hint)).ok();
            }
        }

        resp
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct ButtonBuilder<'a> {
    label: &'a str,
    action: Option<Box<dyn Fn() + 'a>>,
    hint: Option<&'a str>,
    is_enabled_fn: Option<Box<dyn Fn() -> bool + 'a>>,
    hint_tx: SyncSender<String>,
}

impl<'a> ButtonBuilder<'a> {
    #[must_use]
    pub fn new(label: &'a str, hint_tx: SyncSender<String>) -> Self {
        Self {
            label,
            action: None,
            hint: None,
            is_enabled_fn: None,
            hint_tx,
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
    pub fn hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
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
            hint: builder.hint,
            is_enabled_fn: builder.is_enabled_fn,
            hint_tx: builder.hint_tx,
        }
    }
}

