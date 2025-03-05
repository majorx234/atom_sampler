use atom_sampler_lib::{
    atom_event::AtomEvent,
    jackprocess::{start_jack_thread, JackRingBuffer},
    ui::elements::DebugConsole,
    wave_manager::start_wave_manager,
};
use eframe::{self, egui::ViewportBuilder};
mod atom_sampler_app;
use atom_sampler_app::AtomSamplerApp;
use atom_sampler_lib::jackmidi::MidiMsgGeneric;
use bus::Bus;
use ringbuf::{traits::Split, HeapRb};

fn main() {
    println!("atom sampler WIP!");
    let ringbuffer_left_visual = HeapRb::<(f32, f32)>::new(375);
    let ringbuffer_right_visual = HeapRb::<(f32, f32)>::new(375);

    let (ringbuffer_left_visual_in, ringbuffer_left_visual_out) = ringbuffer_left_visual.split();
    let (ringbuffer_right_visual_in, ringbuffer_right_visual_out) = ringbuffer_right_visual.split();

    let mut tx_close = Bus::<bool>::new(1);
    let rx1_close = tx_close.add_rx();
    let rx2_close = tx_close.add_rx();

    let mut tx_atom_event = Bus::<AtomEvent>::new(1);
    let rx1_atom_event = tx_atom_event.add_rx();
    let rx2_atom_event = tx_atom_event.add_rx();

    //let (tx_atom_event, rx_atom_event) = unbounded();

    let mut tx_midi = Bus::<MidiMsgGeneric>::new(20);
    let rx1_midi = tx_midi.add_rx();
    let rx2_midi = tx_midi.add_rx();

    let (jack_ringbuffer, jack_join_handle) = start_jack_thread(rx1_close, tx_midi, rx1_atom_event);

    start_wave_manager(
        rx2_close,
        jack_ringbuffer,
        ringbuffer_left_visual_in,
        ringbuffer_right_visual_in,
        rx2_atom_event,
    );

    let msgs: Vec<String> = Vec::new();
    let n_items = msgs.len();
    let atom_sampler_app = AtomSamplerApp::new(
        false,
        false,
        DebugConsole { msgs, n_items },
        tx_close,
        tx_atom_event,
        Some(ringbuffer_left_visual_out),
        Some(ringbuffer_right_visual_out),
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
    let _ = jack_join_handle.join();
}
