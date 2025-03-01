use crate::wave::play::start_playback;
use crate::wave::record::start_recording;
use crate::{dsp::sample::Sample, wave_commands::WaveCommands};
use bus::Bus;
use crossbeam_channel::Receiver;
use ringbuf::{HeapCons, HeapProd};
use std::path::PathBuf;
use std::thread;

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
    pub tx_stop_rec_bus: Option<Bus<bool>>,
    pub tx_stop_play_bus: Option<Bus<bool>>,
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
            tx_stop_rec_bus: None,
            tx_stop_play_bus: None,
        }
    }
    // WIP replacement for logic in wave_manager
    pub fn start_recording(
        &mut self,
        ringbuffer_left_in_opt: &mut Option<HeapCons<f32>>,
        ringbuffer_right_in_opt: &mut Option<HeapCons<f32>>,
        ringbuffer_left_visual_in_opt: &mut Option<HeapProd<(f32, f32)>>,
        ringbuffer_right_visual_in_opt: &mut Option<HeapProd<(f32, f32)>>,
    ) {
        if let (Some(ringbuffer_left_in), Some(ringbuffer_right_in)) = (
            ringbuffer_left_in_opt.take(),
            ringbuffer_right_in_opt.take(),
        ) {
            self.tx_stop_rec_bus = Some(Bus::<bool>::new(1));
            let rx1_stop_rec = self.tx_stop_rec_bus.as_mut().unwrap().add_rx();
            if let (Some(ringbuffer_left_visual_in), Some(ringbuffer_right_visual_in)) = (
                ringbuffer_left_visual_in_opt.take(),
                ringbuffer_right_visual_in_opt.take(),
            ) {
                self.rec_processing = Some(start_recording(
                    ringbuffer_left_in,
                    ringbuffer_right_in,
                    Some(ringbuffer_left_visual_in),
                    Some(ringbuffer_right_visual_in),
                    rx1_stop_rec,
                ));
            } else {
                self.rec_processing = Some(start_recording(
                    ringbuffer_left_in,
                    ringbuffer_right_in,
                    None,
                    None,
                    rx1_stop_rec,
                ));
            }
            self.state_recording = true;
        }
    }

    pub fn stop_recording(&mut self) {
        if let Some(ref mut tx_stop_rec) = self.tx_stop_rec_bus {
            let _ = tx_stop_rec.try_broadcast(false);
        }
    }

    pub fn get_recording(&mut self) {
        if let Some(recording_join_handle) = self.rec_processing.take() {
            if recording_join_handle.is_finished() {
                if let Ok((left_data, right_data)) = recording_join_handle.join() {
                    // create sample
                    let mut recording = Sample::new();
                    let _ = recording.load_from_data(left_data, right_data);
                    self.sample = Some(recording);
                }
                self.state_recording = false;
            } else {
                self.rec_processing = Some(recording_join_handle);
            }
        }
    }

    pub fn start_playback(
        &mut self,
        ringbuffer_left_out_opt: &mut Option<HeapProd<f32>>,
        ringbuffer_right_out_opt: &mut Option<HeapProd<f32>>,
    ) {
        if let Some(sample) = self.sample.take() {
            if let (Some(ringbuffer_left_out), Some(ringbuffer_right_out)) = (
                ringbuffer_left_out_opt.take(),
                ringbuffer_right_out_opt.take(),
            ) {
                self.tx_stop_play_bus = Some(Bus::<bool>::new(1));
                let rx1_stop_play = self.tx_stop_play_bus.as_mut().unwrap().add_rx();

                self.play_processing = Some(start_playback(
                    ringbuffer_left_out,
                    ringbuffer_right_out,
                    rx1_stop_play,
                    sample,
                ));
            }
        }
    }
    pub fn stop_playback(&mut self) {
        if let Some(ref mut tx_stop_play) = self.tx_stop_play_bus {
            let _ = tx_stop_play.try_broadcast(false);
        }
    }
    pub fn get_playback_finished(&mut self) {
        // TODO: restart playback with message
        if let Some(playback_join_handle) = self.play_processing.take() {
            if playback_join_handle.is_finished() {
                self.state_playback = false;
            } else {
                self.play_processing = Some(playback_join_handle);
            }
        }
    }
    pub fn change_start_address(&mut self, address: usize) {
        self.start_address = address;
    }
    pub fn change_end_address(&mut self, address: usize) {
        self.end_address = address;
    }
}
