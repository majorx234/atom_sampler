use atom_sampler_lib::{atom_event, jackprocess::start_jack_thread, ui::elements::DebugConsole};
use eframe::{self, egui::ViewportBuilder};
mod atom_sampler_app;
use atom_sampler_app::AtomSamplerApp;
use atom_sampler_lib::jackmidi::MidiMsgGeneric;
use bus::Bus;
use crossbeam_channel::{unbounded, Receiver, Sender};
use ringbuf::{traits::Split, HeapRb};

fn main() {
    println!("atom sampler WIP!");
    let ringbuffer_left = HeapRb::<f32>::new(192000);
    let ringbuffer_right = HeapRb::<f32>::new(192000);

    let ringbuffer_left_play = HeapRb::<f32>::new(192000);
    let ringbuffer_right_play = HeapRb::<f32>::new(192000);

    let (ringbuffer_left_in, ringbuffer_left_out) = ringbuffer_left.split();
    let (ringbuffer_right_in, ringbuffer_right_out) = ringbuffer_right.split();

    let (ringbuffer_left_play_in, ringbuffer_left_play_out) = ringbuffer_left_play.split();
    let (ringbuffer_right_play_in, ringbuffer_right_play_out) = ringbuffer_right_play.split();

    let mut tx_close = Bus::<bool>::new(1);
    let rx1_close = tx_close.add_rx();
    let (tx_atom_event, rx_atom_event) = unbounded();

    let mut tx_midi = Bus::<MidiMsgGeneric>::new(20);
    let rx1_midi = tx_midi.add_rx();
    let rx2_midi = tx_midi.add_rx();

    let jack_thread = start_jack_thread(
        rx1_close,
        ringbuffer_left_in,
        ringbuffer_right_in,
        ringbuffer_left_play_out,
        ringbuffer_right_play_out,
        tx_midi,
        rx_atom_event,
    );

    let msgs: Vec<String> = Vec::new();
    let n_items = msgs.len();
    let atom_sampler_app = AtomSamplerApp::new(
        false,
        DebugConsole { msgs, n_items },
        tx_close,
        tx_atom_event,
        Some(ringbuffer_left_out),
        Some(ringbuffer_right_out),
    );

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Atom Sampler MockupGUI",
        options,
        Box::new(|_cc| Box::new(atom_sampler_app)),
    );
    let _ = jack_thread.join();
}
