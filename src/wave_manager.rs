use crate::atom_event::AtomEvent;
use crate::atom_event::Type;
use crate::dsp::sample::Sample;
use crate::wave::play::start_playback;
use crate::wave::record::start_recording;
use crate::wave_handler::WaveHandler;
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
        let mut wave_handler = WaveHandler::new(None);
        let mut tx_stop_rec_opt: Option<bus::Bus<bool>> = None;
        let mut tx_stop_play_opt: Option<bus::Bus<bool>> = None;

        let mut ringbuffer_left_in_opt = Some(ringbuffer_left_in);
        let mut ringbuffer_right_in_opt = Some(ringbuffer_right_in);

        let mut ringbuffer_left_out_opt = Some(ringbuffer_left_out);
        let mut ringbuffer_right_out_opt = Some(ringbuffer_right_out);

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

                                wave_handler.rec_processing = Some(start_recording(
                                    ringbuffer_left_in,
                                    ringbuffer_right_in,
                                    rx1_stop_rec,
                                ));
                                wave_handler.state_recording = true;
                            }
                        } else {
                            // stop recording
                            if let Some(mut tx_stop_rec) = tx_stop_rec_opt.take() {
                                let _ = tx_stop_rec.try_broadcast(false);
                            }
                        }
                    }
                    Type::Playback(state) => {
                        wave_handler.state_playback = state;
                        if wave_handler.state_playback {
                            if let Some(sampple) = wave_handler.sample.take() {
                                if let (Some(ringbuffer_left_out), Some(ringbuffer_right_out)) = (
                                    ringbuffer_left_out_opt.take(),
                                    ringbuffer_right_out_opt.take(),
                                ) {
                                    tx_stop_play_opt = Some(Bus::<bool>::new(1));
                                    let rx1_stop_play = tx_stop_play_opt.as_mut().unwrap().add_rx();

                                    wave_handler.play_processing = Some(start_playback(
                                        ringbuffer_left_out,
                                        ringbuffer_right_out,
                                        rx1_stop_play,
                                        sampple,
                                    ));
                                }
                            } else {
                                // stop playback
                                if let Some(mut tx_stop_play) = tx_stop_play_opt.take() {
                                    let _ = tx_stop_play.try_broadcast(false);
                                }
                            }
                        }
                    }
                    Type::ChangeStartAdress(adress) => {
                        wave_handler.start_address = adress;
                    }
                    Type::ChangeEndAdress(adress) => {
                        wave_handler.end_address = adress;
                    }
                }
            }

            if wave_handler.state_recording {
                if let Some(recording_join_handle) = wave_handler.rec_processing.take() {
                    if recording_join_handle.is_finished() {
                        if let Ok((left_data, right_data)) = recording_join_handle.join() {
                            // create sample
                            let mut recording = Sample::new();
                            let _ = recording.load_from_data(left_data, right_data);
                            wave_handler.sample = Some(recording);
                        }
                        wave_handler.state_recording = false;
                    } else {
                        wave_handler.rec_processing = Some(recording_join_handle);
                    }
                }
            }
            if wave_handler.state_playback {
                // TODO: restart playback with message
                if let Some(playback_join_handle) = wave_handler.play_processing.take() {
                    if playback_join_handle.is_finished() {
                        wave_handler.state_playback = false;
                    } else {
                        wave_handler.play_processing = Some(playback_join_handle);
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
