use crate::{dsp::sample::Sample, wave_commands::WaveCommands};
use crossbeam_channel::Receiver;
use std::path::PathBuf;
use std::{process::exit, thread, time::Duration};

struct WaveHandler {
    sample: Option<Sample>,
    is_loaded: bool,
    state_recording: bool,
    state_playback: bool,
    path: Option<PathBuf>,
    rx_command: Receiver<WaveCommands>,
    processing: Option<thread::JoinHandle<()>>,
}

impl WaveHandler {
    pub fn new(rx_command: Receiver<WaveCommands>) -> Self {
        WaveHandler {
            sample: None,
            is_loaded: false,
            state_recording: false,
            state_playback: false,
            path: None,
            rx_command,
            processing: None,
        }
    }
}
