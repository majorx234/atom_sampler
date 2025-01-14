use crate::atom_event::AtomEvent;
use crate::atom_event::Type;
use bus::{Bus, BusReader};
use ringbuf::{
    traits::{Consumer, Observer},
    HeapCons, HeapProd,
};
use std::{process::exit, thread, time::Duration};

pub fn start_wave_manager(
    mut rx_close: BusReader<bool>,
    mut ringbuffer_left_in: HeapCons<f32>,
    mut ringbuffer_right_in: HeapCons<f32>,
    mut ringbuffer_left_out: HeapProd<f32>,
    mut ringbuffer_right_out: HeapProd<f32>,
    mut rx_atom_event: BusReader<AtomEvent>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let mut state_recording = false;
        let mut state_playback = false;
        let mut start_address = 0;
        let mut end_address = 0;
        let mut tx_stop_rec_opt: Option<bus::Bus<bool>> = None;
        let mut tx_stop_play_opt: Option<bus::Bus<bool>> = None;

        let mut ringbuffer_left_in_opt = Some(ringbuffer_left_in);
        let mut ringbuffer_right_in_opt = Some(ringbuffer_right_in);

        let mut ringbuffer_left_out_opt = Some(ringbuffer_left_out);
        let mut ringbuffer_right_out_opt = Some(ringbuffer_right_out);

        let mut recording_join_handle_opt: Option<_> = None;
        let mut playback_join_handle_opt: Option<_> = None;
        while run {
            let opt_atom_event: Option<AtomEvent> =
                if let Ok(rx_atome_event) = rx_atom_event.try_recv() {
                    Some(rx_atome_event)
                } else {
                    None
                };
            if let Some(atom_event) = opt_atom_event {
                match atom_event.event_type {
                    Type::Recording(state) => {
                        if state {
                            // start recording
                            if let (Some(ringbuffer_left_in), Some(ringbuffer_right_in)) = (
                                ringbuffer_left_in_opt.take(),
                                ringbuffer_right_in_opt.take(),
                            ) {
                                tx_stop_rec_opt = Some(Bus::<bool>::new(1));
                                let rx1_stop_rec = tx_stop_rec_opt.as_mut().unwrap().add_rx();

                                recording_join_handle_opt = Some(start_recording(
                                    ringbuffer_left_in,
                                    ringbuffer_right_in,
                                    rx1_stop_rec,
                                ));
                                state_recording = state;
                            }
                        } else {
                            // stop recording
                            if let Some(mut tx_stop_rec) = tx_stop_rec_opt.take() {
                                let _ = tx_stop_rec.try_broadcast(false);
                            }
                        }
                    }
                    Type::Playback(state) => {
                        state_playback = state;
                        if state_playback {
                            if let (Some(ringbuffer_left_out), Some(ringbuffer_right_out)) = (
                                ringbuffer_left_out_opt.take(),
                                ringbuffer_right_out_opt.take(),
                            ) {
                                tx_stop_play_opt = Some(Bus::<bool>::new(1));
                                let rx1_stop_play = tx_stop_play_opt.as_mut().unwrap().add_rx();

                                playback_join_handle_opt = Some(start_playback(
                                    ringbuffer_left_out,
                                    ringbuffer_right_out,
                                    rx1_stop_play,
                                ));
                            }
                        } else {
                            // stop playback
                            if let Some(mut tx_stop_play) = tx_stop_play_opt.take() {
                                let _ = tx_stop_play.try_broadcast(false);
                            }
                        }
                    }
                    Type::ChangeStartAdress(adress) => {
                        start_address = adress;
                    }
                    Type::ChangeEndAdress(adress) => {
                        end_address = adress;
                    }
                }
            }

            if state_recording {
                if let Some(recording_join_handle) = recording_join_handle_opt.take() {
                    if recording_join_handle.is_finished() {
                        if let Ok((_left_data, _right_data)) = recording_join_handle.join() {
                            // create sample
                        }
                        state_recording = false;
                    } else {
                        recording_join_handle_opt = Some(recording_join_handle);
                    }
                }
            }
            if state_playback {
                // TODO: restart playback with message
                if let Some(playback_join_handle) = playback_join_handle_opt.take() {
                    if playback_join_handle.is_finished() {
                        state_playback = false;
                    } else {
                        playback_join_handle_opt = Some(playback_join_handle);
                    }
                }
            }

            // TODO: have better wait on msgs to receive instead of polling pattern
            thread::sleep(Duration::from_millis(100));
            match rx_close.recv() {
                Ok(running) => run = running,
                Err(_) => run = false,
            }
        }
    })
}

fn start_recording(
    mut ringbuffer_left_in: HeapCons<f32>,
    mut ringbuffer_right_in: HeapCons<f32>,
    mut rx_stop_rec: BusReader<bool>,
) -> std::thread::JoinHandle<(Vec<f32>, Vec<f32>)> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let wave_size = 192000;
        let mut wave_left = vec![0.0; wave_size];
        let mut wave_right = vec![0.0; wave_size];
        let mut vecpointer_left = 0;
        let mut vecpointer_right = 0;
        while run {
            let length_left = 1024.min(ringbuffer_left_in.occupied_len());
            let length_right = 1024.min(ringbuffer_right_in.occupied_len());

            if (vecpointer_left + length_left < wave_size)
                && (vecpointer_right + length_right < wave_size)
            {
                wave_left.splice(vecpointer_left..length_left, ringbuffer_left_in.pop_iter());
                vecpointer_left += length_left;
                wave_right.splice(
                    vecpointer_right..length_right,
                    ringbuffer_right_in.pop_iter(),
                );
                vecpointer_right += length_right;
            } else {
                ringbuffer_left_in.clear();
                ringbuffer_right_in.clear();
                run = false;
            }
            if let Ok(is_stop) = rx_stop_rec.try_recv() {
                if is_stop {
                    run = false;
                }
            }
        }
        (wave_left, wave_right)
    })
}

fn start_playback(
    mut ringbuffer_left_out: HeapProd<f32>,
    mut ringbuffer_right_out: HeapProd<f32>,
    mut rx_stop_play: BusReader<bool>,
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
