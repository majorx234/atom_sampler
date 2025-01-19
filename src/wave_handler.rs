use crate::wave::play::start_playback;
use crate::wave::record::start_recording;
use crate::{dsp::sample::Sample, wave_commands::WaveCommands};
use bus::{Bus, BusReader};
use crossbeam_channel::Receiver;
use ringbuf::{
    traits::{Consumer, Observer},
    HeapCons, HeapProd,
};
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
    pub tx_stop_bus: Option<Bus<bool>>,
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
            tx_stop_bus: None,
        }
    }
    // WIP replacement for logic in wave_manager
    pub fn start_recording(
        &mut self,
        mut ringbuffer_left_in_opt: Option<HeapCons<f32>>,
        mut ringbuffer_right_in_opt: Option<HeapCons<f32>>,
    ) {
        if let (Some(ringbuffer_left_in), Some(ringbuffer_right_in)) = (
            ringbuffer_left_in_opt.take(),
            ringbuffer_right_in_opt.take(),
        ) {
            self.tx_stop_bus = Some(Bus::<bool>::new(1));
            let rx1_stop_rec = self.tx_stop_bus.as_mut().unwrap().add_rx();

            self.rec_processing = Some(start_recording(
                ringbuffer_left_in,
                ringbuffer_right_in,
                rx1_stop_rec,
            ));
            self.state_recording = true;
        }
    }

    pub fn stop_recording(&mut self) {
        if let Some(mut tx_stop_rec) = self.tx_stop_bus.take() {
            let _ = tx_stop_rec.try_broadcast(false);
        }
    }
}
