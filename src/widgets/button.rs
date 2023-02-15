use eframe::egui::{
    self,
    Widget,
    Ui,
    Response,
};
use std::{
    sync::{
        Arc,
        Mutex,
        mpsc::{SyncSender, TrySendError},
    },
    path::PathBuf,
    process::Command,
};
use crate::{
    traits::{
        LogIfErr,
        LockIgnorePoisoned, PushChained,
    },
    widgets::ModListing,
    writer_thread,
    CHANGED_ACTIVE_MODS,
    helpers::{config::get_mod_list_path, paths::path_to_str},
    ModsConfig,
};

/// The buttons that appear to the right of the mod lists.
pub struct Button<'a> {
    label: &'a str,
    action: Option<Box<dyn Fn() + 'a>>,
    is_enabled_fn: Option<Box<dyn Fn() -> bool + 'a>>,
    hint_sender: Option<HintSender<'a>>,
}

impl std::fmt::Debug for Button<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Button {{ label: {:?}, hint: {:?} }}", self.label, self.hint_sender.as_ref().map(|h| h.msg)))
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
        self.is_enabled_fn.as_ref()
            .map_or(true, |f| f())
    }

    /// Generates the [`Button`] that clears the active mod list.
    #[must_use]
    pub fn clear(hint_tx: SyncSender<String>) -> Self {
        let action = Box::new(|| log::debug!("Unimplemented 😇")) as Box<dyn Fn() + 'a>;
        let hint = "Remove all mods, except Core and DLCs";

        Self::builder("Clear")
            .action(action)
            .hint(hint, hint_tx)
            .build()
    }

    /// Generates the [`Button`] that auto-sorts the active mod list.
    #[must_use]
    pub fn sort(hint_tx: SyncSender<String>) -> Self {
        let action = Box::new(|| log::debug!("Unimplemented 😇")) as Box<dyn Fn() + 'a>;
        let hint = "Auto-sort mods";

        Self::builder("Sort")
            .action(action)
            .hint(hint, hint_tx)
            .build()
    }

    /// Generates the [`Button`] that saves the active mod list to disk.
    #[must_use]
    pub fn save(hint_tx: SyncSender<String>, writer_thread_tx: SyncSender<writer_thread::Message>, active_mod_listing_ref: Arc<Mutex<ModListing<'a>>>) -> Self {
        let action = Box::new(move || {
            let active_mods: Vec<String> = Vec::from(&*active_mod_listing_ref.clone().lock_ignore_poisoned());
            writer_thread_tx.try_send(writer_thread::Message::SetActiveMods(active_mods))
                .and_then(|_| writer_thread_tx.try_send(writer_thread::Message::Save))
                .log_if_err();
        }) as Box<dyn Fn() + 'a>;
        let hint = "Save the mod list to ModsConfig.xml file (applies changes to game mod list)";
        let is_enabled = Box::new(|| CHANGED_ACTIVE_MODS.check()) as Box<dyn Fn() -> bool + 'a>;

        Self::builder("Save")
            .action(action)
            .hint(hint, hint_tx)
            .is_enabled_fn(is_enabled)
            .build()
    }

    /// Generates the [`Button`] that launches the game.
    #[must_use]
    pub fn run(hint_tx: SyncSender<String>, exe_path: PathBuf, args: Option<String>) -> Self {
        let action = Box::new(move || {
            let mut cmd = Command::new(&exe_path);
            if let Some(args) = args.as_ref() {
                cmd.arg(args); // idk if this'll work with more complex args than I use, TODO check
            }
            cmd.spawn()
                .log_if_err();
        }) as Box<dyn Fn() + 'a>;
        let hint = "Run the game";
        let is_enabled = Box::new(|| !CHANGED_ACTIVE_MODS.check()) as Box<dyn Fn() -> bool>;

        Self::builder("Run")
            .action(action)
            .hint(hint, hint_tx)
            .is_enabled_fn(is_enabled)
            .build()
    }

    #[must_use]
    pub fn import_list(hint_tx: SyncSender<String>, change_mod_list_tx: SyncSender<Vec<String>>) -> Self {
        let hint = "Imports mod list from mod list file";
        let action = Box::new(move || {
            let path = get_mod_list_path().log_if_err()
                .map(|p| p.push_chained("")); // need to push empty so it opens in the dir rather than in its parent with the dir name as the input
            let path = path.as_ref()
                .and_then(path_to_str)
                .unwrap_or_default();
            if let Some(parsed) = tinyfiledialogs::open_file_dialog("Select mod list", path, Some((&["*.xml"], "")))
                .and_then(|p| ModsConfig::try_from(PathBuf::from(p).as_path()).log_if_err())
            {
                change_mod_list_tx.try_send(parsed.activeMods)
                    .log_if_err();
            }
        }) as Box<dyn Fn() + 'a>;

        Self::builder("Import list")
            .hint(hint, hint_tx)
            .action(action)
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
            self.hint_sender.as_ref()
                .map(HintSender::try_send);
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

struct HintSender<'a> {
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
            Err(TrySendError::Full(_)) => { log::warn!("Hint channel full") }
            Err(TrySendError::Disconnected(_)) => { log::error!("Hint mpsc channel unexpectedly disconnected") }
        }
    }
}

