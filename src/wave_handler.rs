use crate::wave_commands::WaveCommands;
use crossbeam_channel::Receiver;
use std::path::PathBuf;
use std::{process::exit, thread, time::Duration};

struct WaveHandler {
    is_loaded: bool,
    path: Option<PathBuf>,
    rx_command: Receiver<WaveCommands>,
    processing: thread::JoinHandle<()>,
}
