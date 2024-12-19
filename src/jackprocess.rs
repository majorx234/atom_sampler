use crate::atom_event::AtomEvent;
use crate::atom_event::Type;
use crate::jackmidi::MidiMsgGeneric;
use bus::{Bus, BusReader};
use crossbeam_channel::Receiver;
use jack;
use ringbuf::Consumer;
use ringbuf::Producer;
use ringbuf::SharedRb;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::{process::exit, thread, time::Duration};

pub fn start_jack_thread(
    mut rx_close: BusReader<bool>,
    mut ringbuffer_left_in: Producer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>,
    mut ringbuffer_right_in: Producer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>,
    mut ringbuffer_left_out: Consumer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>,
    mut ringbuffer_right_out: Consumer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>,
    mut midi_sender: Bus<MidiMsgGeneric>,
    rx_atom_event: Receiver<AtomEvent>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let (client, _status) =
            jack::Client::new("atom sampler", jack::ClientOptions::NO_START_SERVER)
                .expect("No Jack server running\n");
        let sample_rate = client.sample_rate();
        // register ports:
        let mut out_a = client.register_port("as_out_l", jack::AudioOut).unwrap();
        let mut out_b = client.register_port("as_out_r", jack::AudioOut).unwrap();
        // register midi ports:
        let midi_in = client.register_port("midi_in", jack::MidiIn).unwrap();

        let in_a = client
            .register_port("as_in_l", jack::AudioIn::default())
            .unwrap();
        let in_b = client
            .register_port("as_in_r", jack::AudioIn::default())
            .unwrap();
        let _midi_in = client.register_port("as_midi_in", jack::MidiIn).unwrap();
        let mut frame_size = client.buffer_size() as usize;
        if client.set_buffer_size(frame_size as u32).is_ok() {
            // get frame size
            let frame_size = client.buffer_size() as usize;
            println!(
                "client started with samplerate: {} and frame_size: {}",
                sample_rate, frame_size
            );
        } else {
            exit(-1);
        }
        if client.set_buffer_size(frame_size as u32).is_ok() {
            // get frame size
            frame_size = client.buffer_size() as usize;
            println!(
                "client started with samplerate: {} and frame_size: {}",
                sample_rate, frame_size
            );
        } else {
            exit(-1);
        }

        // state section
        let mut state_recording = false;
        let mut state_playback = false;

        let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let out_a_p = out_a.as_mut_slice(ps);
            let out_b_p = out_b.as_mut_slice(ps);
            let in_a_p = in_a.as_slice(ps);
            let in_b_p = in_b.as_slice(ps);

            let in_midi_iter = midi_in.iter(ps);

            let opt_atom_event: Option<AtomEvent> =
                if let Ok(rx_atome_event) = rx_atom_event.try_recv() {
                    Some(rx_atome_event)
                } else {
                    None
                };

            // read midi messages
            let midi_in_iter = midi_in.iter(ps);
            for raw_midi_event in midi_in_iter {
                let midi_msg: MidiMsgGeneric = raw_midi_event.into();
                let _ = midi_sender.try_send(midi_msg);
            }

            // setup states with event
            if let Some(atom_event) = opt_atom_event {
                match atom_event.event_type {
                    Type::Recording(state) => {
                        state_recording = state;
                    }
                    Type::Playback(state) => {
                        state_playback = state;
                    }
                    Type::ChangeStartAdress(adress) => {}
                    Type::ChangeEndAdress(adress) => {}
                }
            }

            // zero the ringbuffer
            out_a_p.fill(0.0);
            out_b_p.fill(0.0);

            if state_recording {
                ringbuffer_left_in.push_iter(&mut in_a_p.iter().copied());
                ringbuffer_right_in.push_iter(&mut in_b_p.iter().copied());
            }
            if state_playback {
                ringbuffer_left_out.pop_slice(out_a_p);
                ringbuffer_right_out.pop_slice(out_b_p);
            }

            jack::Control::Continue
        };
        let process = jack::ClosureProcessHandler::new(process_callback);
        let active_client = client.activate_async((), process).unwrap();
        while run {
            thread::sleep(Duration::from_millis(100));
            match rx_close.recv() {
                Ok(running) => run = running,
                Err(_) => run = false,
            }
        }
        match active_client.deactivate() {
            Ok(_) => println!("exit jackaudio thread\n"),
            Err(_) => println!("exit jackaudio thread,client deactivation err\n"),
        }
    })
}
