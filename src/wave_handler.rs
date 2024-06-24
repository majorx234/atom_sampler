use crate::wave_commands::WaveCommands;
use crossbeam_channel::Receiver;
use std::path::PathBuf;
use std::{process::exit, thread, time::Duration};

struct WaveHandler {
    is_loaded: bool,
    path: Option<PathBuf>,
    rx_command: Receiver<WaveCommands>,
    processing: Option<thread::JoinHandle<()>>,
}

impl WaveHandler {
    pub fn new(rx_command: Receiver<WaveCommands>) -> Self {
        WaveHandler {
            is_loaded: false,
            path: None,
            rx_command,
            processing: None,
        }
    }
}
