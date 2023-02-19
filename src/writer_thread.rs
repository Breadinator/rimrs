use crate::{traits::LogIfErr, ModsConfig, CHANGED_ACTIVE_MODS};
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    sync::{
        mpsc::{Receiver, SyncSender},
        Arc,
    },
    thread::{self, JoinHandle},
};

#[derive(Debug, Clone)]
pub enum Message {
    Stop,
    Save,
    SetDestination(PathBuf),
    SetModsConfig(Arc<ModsConfig>),
    SetActiveMods(Vec<String>),
    SetHintTx(SyncSender<String>),
    WriteTo(PathBuf, Vec<String>),
}

#[must_use]
pub fn spawn(rx: Receiver<Message>) -> JoinHandle<()> {
    thread::spawn(move || writer_thread_fn(rx))
}

#[allow(clippy::needless_pass_by_value)] // i want this thread to have full control of the receiver
fn writer_thread_fn(rx: Receiver<Message>) {
    let mut destination: Option<PathBuf> = None;
    let mut mods_config: Option<ModsConfig> = None;
    let mut hint_tx: Option<SyncSender<String>> = None;

    loop {
        match rx.recv() {
            Ok(Message::Save) => write_to(
                destination.as_ref().unwrap(),
                mods_config.as_ref().unwrap(),
                &hint_tx,
                true,
            ),
            Ok(Message::SetDestination(new_dest)) => destination = Some(new_dest),
            Ok(Message::SetModsConfig(new_mods_config)) => {
                mods_config = Some((*new_mods_config).clone());
            }
            Ok(Message::SetActiveMods(new_mods)) => set_active_mods(&mut mods_config, new_mods),
            Ok(Message::SetHintTx(new_hint_tx)) => hint_tx = Some(new_hint_tx),
            Ok(Message::WriteTo(path, mods)) => {
                let mut mods_config = mods_config.clone();
                set_active_mods(&mut mods_config, mods);
                write_to(path, mods_config.as_ref().unwrap(), &hint_tx, false);
            }
            Ok(Message::Stop) => break,
            Err(err) => panic!("{err}"),
        }
    }
}

fn set_active_mods(mods_config: &mut Option<ModsConfig>, new_mods: Vec<String>) {
    if let Some(mc) = mods_config.as_mut() {
        mc.activeMods = new_mods;
    }
}

fn write_to<P: AsRef<Path>>(
    destination: P,
    mods_config: &ModsConfig,
    hint_tx: &Option<SyncSender<String>>,
    reset_flag: bool,
) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(destination.as_ref());

    match file.and_then(|mut f| f.write_all(&Vec::from(mods_config))) {
        Ok(_) => {
            if reset_flag {
                CHANGED_ACTIVE_MODS.reset();
            }
            log::info!("Wrote to {:?}", destination.as_ref());
            if let Some(hint_tx) = hint_tx.as_ref() {
                hint_tx
                    .try_send(format!("Wrote to {:?}", destination.as_ref()))
                    .log_if_err();
            }
        }
        Err(err) => {
            log::error!("{err}");
            if let Some(hint_tx) = hint_tx.as_ref() {
                hint_tx
                    .try_send(format!("Couldn't write to {:?}", destination.as_ref()))
                    .log_if_err();
            }
        }
    }
}
