use crate::atom_event::AtomEvent;
use crate::atom_event::Type;
use crate::wave_handler::WaveHandler;
use bus::BusReader;
use ringbuf::{HeapCons, HeapProd};
use std::{thread, time::Duration};

pub fn start_wave_manager(
    mut rx_close: BusReader<bool>,
    ringbuffer_left_in: HeapCons<f32>,
    ringbuffer_right_in: HeapCons<f32>,
    ringbuffer_left_out: HeapProd<f32>,
    ringbuffer_right_out: HeapProd<f32>,
    mut rx_atom_event: BusReader<AtomEvent>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let mut wave_handler = WaveHandler::new(None);
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
                            wave_handler.start_recording(
                                &mut ringbuffer_left_in_opt.take(),
                                &mut ringbuffer_right_in_opt.take(),
                            );
                        } else {
                            wave_handler.stop_recording();
                        }
                    }
                    Type::Playback(state) => {
                        if state {
                            wave_handler.start_playback(
                                &mut ringbuffer_left_out_opt.take(),
                                &mut ringbuffer_right_out_opt.take(),
                            );
                            wave_handler.state_playback = state;
                        } else {
                            wave_handler.stop_playback();
                        }
                    }
                    Type::ChangeStartAdress(address) => {
                        wave_handler.change_start_address(address);
                    }
                    Type::ChangeEndAdress(address) => {
                        wave_handler.change_end_address(address);
                    }
                }
            }

            if wave_handler.state_recording {
                wave_handler.get_recording();
            }
            if wave_handler.state_playback {
                wave_handler.get_playback_finished();
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
