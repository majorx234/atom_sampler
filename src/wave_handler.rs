use crate::{dsp::sample::Sample, wave_commands::WaveCommands};
use crossbeam_channel::Receiver;
use std::path::PathBuf;
use std::{process::exit, thread, time::Duration};

pub struct WaveHandler {
    pub sample: Option<Sample>,
    pub start_address: usize,
    pub end_address: usize,
    pub is_loaded: bool,
    pub state_recording: bool,
    pub state_playback: bool,
    pub path: Option<PathBuf>,
    pub rx_command: Option<Receiver<WaveCommands>>,
    pub rec_processing: Option<thread::JoinHandle<(Vec<f32>, Vec<f32>)>>,
    pub play_processing: Option<thread::JoinHandle<()>>,
}

impl WaveHandler {
    pub fn new(rx_command: Option<Receiver<WaveCommands>>) -> Self {
        WaveHandler {
            sample: None,
            start_address: 0,
            end_address: 0,
            is_loaded: false,
            state_recording: false,
            state_playback: false,
            path: None,
            rx_command,
            rec_processing: None,
            play_processing: None,
        }
    }
}
