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
    ringbuffer_left_visual_out: HeapProd<(f32, f32)>,
    ringbuffer_right_visual_out: HeapProd<(f32, f32)>,
    mut rx_atom_event: BusReader<AtomEvent>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let mut wave_handler = WaveHandler::new(None);
        let mut ringbuffer_left_in_opt = Some(ringbuffer_left_in);
        let mut ringbuffer_right_in_opt = Some(ringbuffer_right_in);
        let mut ringbuffer_left_out_opt = Some(ringbuffer_left_out);
        let mut ringbuffer_right_out_opt = Some(ringbuffer_right_out);
        let mut ringbuffer_left_visual_out_opt = Some(ringbuffer_left_visual_out);
        let mut ringbuffer_right_visual_out_opt = Some(ringbuffer_right_visual_out);

        while run {
            let opt_atom_event: Option<AtomEvent> = if let Ok(rx_atome_event) =
                rx_atom_event.recv_timeout(Duration::from_millis(100))
            {
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
                                &mut ringbuffer_left_visual_out_opt.take(),
                                &mut ringbuffer_right_visual_out_opt.take(),
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
                let ringbufs_opt = wave_handler.get_recording();
                if let Some(ringbufs) = ringbufs_opt {
                    ringbuffer_left_in_opt = Some(ringbufs.0);
                    ringbuffer_right_in_opt = Some(ringbufs.1);
                    ringbuffer_left_visual_out_opt = ringbufs.2;
                    ringbuffer_right_visual_out_opt = ringbufs.3;
                }
            }
            if wave_handler.state_playback {
                let ringbufs_opt = wave_handler.get_playback_finished();
                if let Some(ringbufs) = ringbufs_opt {
                    ringbuffer_left_out_opt = Some(ringbufs.0);
                    ringbuffer_right_out_opt = Some(ringbufs.1);
                }
            }
            match rx_close.try_recv() {
                Ok(running) => run = running,
                Err(err) => {
                    println!("close err: {:?}", err);
                }
            }
        }
    })
}
