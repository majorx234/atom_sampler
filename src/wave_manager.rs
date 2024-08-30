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
    rx_atom_event: BusReader<AtomEvent>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        while run {
            thread::sleep(Duration::from_millis(100));
            match rx_close.recv() {
                Ok(running) => run = running,
                Err(_) => run = false,
            }
        }
    })
}
