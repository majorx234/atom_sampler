use atom_sampler_lib::{jackprocess::start_jack_thread, ui::elements::DebugConsole};
use eframe::{self, egui::ViewportBuilder};
mod atom_sampler_app;
use atom_sampler_app::AtomSamplerApp;
use bus::Bus;
use crossbeam_channel::{unbounded, Receiver, Sender};
use ringbuf::HeapRb;

fn main() {
    println!("atom sampler WIP!");
    let ringbuffer_left = HeapRb::<f32>::new(192000);
    let ringbuffer_right = HeapRb::<f32>::new(192000);

    let (ringbuffer_left_in, ringbuffer_left_out) = ringbuffer_left.split();
    let (ringbuffer_right_in, ringbuffer_right_out) = ringbuffer_right.split();

    let mut tx_close = bus::Bus::<bool>::new(1);
    let mut rx1_close = tx_close.add_rx();
    let (tx_atom_event, rx_atom_event) = unbounded();
    let jack_thread = start_jack_thread(
        rx1_close,
        ringbuffer_left_in,
        ringbuffer_right_in,
        rx_atom_event,
    );

    let msgs: Vec<String> = Vec::new();
    let n_items = msgs.len();
    let atom_sampler_app = AtomSamplerApp {
        wave_loaded: false,
        console: DebugConsole { msgs, n_items },
    };

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Atom Sampler MockupGUI",
        options,
        Box::new(|_cc| Box::new(atom_sampler_app)),
    );
}
