use std::{
    thread::{
        self,
        JoinHandle,
    },
    sync::{
        Arc,
        mpsc::Receiver,
    },
    path::{
        Path,
        PathBuf,
    },
};
use crate::ModsConfig;

pub enum Message {
    Stop,
    Write(Vec<String>),
    SetDestination(PathBuf),
    SetModsConfig(Arc<ModsConfig>),
}

#[must_use]
pub fn spawn(rx: Receiver<Message>) -> JoinHandle<()> {
    thread::spawn(move || writer_thread_fn(rx))
}

#[allow(clippy::needless_pass_by_value)] // i want this thread to have full control of the receiver
fn writer_thread_fn(rx: Receiver<Message>) {
    let mut destination: Option<PathBuf> = None;
    let mut mods_config: Option<ModsConfig> = None;

    loop {
        match rx.recv() {
            Ok(Message::Write(active_mods)) => write_to(destination.as_ref().unwrap(), mods_config.as_ref().unwrap(), &active_mods),
            Ok(Message::SetDestination(new_dest)) => destination = Some(new_dest),
            Ok(Message::SetModsConfig(new_mods_config)) => mods_config = Some((*new_mods_config).clone()),
            Ok(Message::Stop) => break,
            Err(err) => panic!("{err}"),
        }
    }
}

fn write_to<P: AsRef<Path>>(destination: P, starting_mods_config: &ModsConfig, active_mods: &[String]) {
    log::debug!("Should write to {:?} now", destination.as_ref());
}

