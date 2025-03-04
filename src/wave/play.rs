use crate::dsp::sample::Sample;
use bus::BusReader;
use ringbuf::{
    traits::{Observer, Producer},
    HeapProd,
};
use std::{thread, time::Duration};

pub fn start_playback(
    mut ringbuffer_left_out: HeapProd<f32>,
    mut ringbuffer_right_out: HeapProd<f32>,
    mut rx_stop_play: BusReader<bool>,
    sample: Sample,
) -> std::thread::JoinHandle<(HeapProd<f32>, HeapProd<f32>)> {
    std::thread::spawn(move || {
        // TODO: playback from buffer
        let mut vecpointer_left = 0;
        let mut vecpointer_right = 0;

        let vec_left_max_len = sample.data_left.len();
        let vec_right_max_len = sample.data_right.len();
        loop {
            if let Ok(is_stop) = rx_stop_play.try_recv() {
                if is_stop {
                    break;
                }
            }
            if vecpointer_left >= vec_left_max_len || vecpointer_right >= vec_right_max_len {
                break;
            }
            // TODO need conditional var here
            if ringbuffer_left_out.occupied_len() < 2048 {
                let data_to_copy_left = (vec_left_max_len - vecpointer_left).min(1024);
                let data_to_copy_right = (vec_right_max_len - vecpointer_right).min(1024);
                vecpointer_left += ringbuffer_left_out.push_slice(
                    &sample.data_left[vecpointer_left..vecpointer_left + data_to_copy_left],
                );
                vecpointer_right += ringbuffer_right_out.push_slice(
                    &sample.data_right[vecpointer_right..vecpointer_right + data_to_copy_right],
                );
            } else {
                // TODO: adjust time according to samples written
                let sleep_time_ms = 0.5 * 48000.0 / 1024.0;
                thread::sleep(Duration::from_millis(sleep_time_ms as u64));
            }
        }
        (ringbuffer_left_out, ringbuffer_right_out)
    })
}
