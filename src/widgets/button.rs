use eframe::egui::{
    self,
    Widget,
    Ui,
    Response,
};

pub struct Button<'a> {
    label: &'a str,
    action: Box<dyn Fn() + 'a>,
    hint: Option<&'a str>,
}

impl std::fmt::Debug for Button<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Button {{ label: {:?}, hint: {:?} }}", self.label, self.hint))
    }
}

impl<'a> Button<'a> {
    #[must_use]
    pub fn new(label: &'a str, action: Box<dyn Fn()>, hint: Option<&'a str>) -> Self {
        Self {
            label,
            action,
            hint,
        }
    }

    #[must_use]
    pub fn clear() -> Self {
        let action = Box::new(|| log::debug!("Unimplemented 😇")) as Box<dyn Fn()>;
        Self::new("Clear", action, Some("Remove all mods, except Core and DLCs."))
    }
}

impl<'a> Widget for &Button<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let btn = egui::Button::new(self.label);
        let resp = ui.add(btn);

        if resp.clicked() {
            (self.action)();
        }

        if resp.hovered() {
            if let Some(hint) = self.hint {
                log::debug!("Tried to show hint, but unimplemented lol.\n        Hint: {hint}");
            }
        }

        resp
    }
}

