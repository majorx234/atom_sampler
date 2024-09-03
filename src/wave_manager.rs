use crate::atom_event::AtomEvent;
use crate::atom_event::Type;
use bus::BusReader;
use ringbuf::Consumer;
use ringbuf::Producer;
use ringbuf::SharedRb;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::{process::exit, thread, time::Duration};

pub fn start_wave_manager(
    mut rx_close: BusReader<bool>,
    mut ringbuffer_left_in: Consumer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>,
    mut ringbuffer_right_in: Consumer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>,
    mut ringbuffer_left_out: Producer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>,
    mut ringbuffer_right_out: Producer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>,
    mut rx_atom_event: BusReader<AtomEvent>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let mut state_recording = false;
        let mut state_playback = false;

        let mut wave_left = vec![0.0; 192000];
        let mut wave_right = vec![0.0; 192000];
        let mut vecpointer_left = 0;
        let mut vecpointer_right = 0;
        let mut start_address = 0;
        let mut end_address = 0;
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
                        state_recording = state;
                    }
                    Type::Playback(state) => {
                        state_playback = state;
                    }
                    Type::ChangeStartAdress(adress) => {
                        start_address = adress;
                    }
                    Type::ChangeEndAdress(adress) => {
                        end_address = adress;
                    }
                }
            }

            if (state_recording) {
                let length_left = ringbuffer_left_in.len();
                wave_left.splice(vecpointer_left..length_left, ringbuffer_left_in.pop_iter());
                let length_right = ringbuffer_right_in.len();
                wave_right.splice(
                    vecpointer_right..length_right,
                    ringbuffer_right_in.pop_iter(),
                );
            }

            thread::sleep(Duration::from_millis(100));
            match rx_close.recv() {
                Ok(running) => run = running,
                Err(_) => run = false,
            }
        }
    })
}
