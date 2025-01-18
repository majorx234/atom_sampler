use crate::dsp::sample::Sample;
use bus::{Bus, BusReader};
use ringbuf::{
    traits::{Consumer, Observer},
    HeapCons, HeapProd,
};
use std::{process::exit, thread, time::Duration};

pub fn start_playback(
    mut ringbuffer_left_out: HeapProd<f32>,
    mut ringbuffer_right_out: HeapProd<f32>,
    mut rx_stop_play: BusReader<bool>,
    mut sample: Sample,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        // TODO: playback from buffer
        let mut vecpointer_left = 0;
        let mut vecpointer_right = 0;
        let mut run = true;
        while run {
            if let Ok(is_stop) = rx_stop_play.try_recv() {
                if is_stop {
                    run = false;
                }
            }

            // TODO: adjust time according to samples written
            let mut sleep_time_ms = 100;
            thread::sleep(Duration::from_millis(sleep_time_ms));
        }
    })
}
