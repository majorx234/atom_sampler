use eframe::{self, egui::ViewportBuilder};
mod wave_record_gui;
use atom_sampler_lib::{
    atom_event::AtomEvent,
    jackmidi::MidiMsgGeneric,
    jackprocess::{start_jack_thread, JackRingBuffer},
    wave_manager::start_wave_manager,
};
use bus::Bus;
use ringbuf::{traits::Split, HeapRb};
use wave_record_gui::WaveRecordGUI;

fn main() {
    let mut tx_close = Bus::<bool>::new(1);
    let rx1_close = tx_close.add_rx();
    let rx2_close = tx_close.add_rx();

    let mut tx_atom_event = Bus::<AtomEvent>::new(1);
    let rx1_atom_event = tx_atom_event.add_rx();
    let rx2_atom_event = tx_atom_event.add_rx();

    let mut tx_midi = Bus::<MidiMsgGeneric>::new(20);
    let rx1_midi = tx_midi.add_rx();
    let rx2_midi = tx_midi.add_rx();

    let (jack_ringbuffer, jack_join_handle) = start_jack_thread(rx1_close, tx_midi, rx1_atom_event);

    let (ringbuffer_left_visual_out, ringbuffer_right_visual_out, wave_manager_join_handle) =
        start_wave_manager(rx2_close, jack_ringbuffer, rx2_atom_event);
    let wave_record_gui = WaveRecordGUI::new(
        Some(ringbuffer_left_visual_out),
        Some(ringbuffer_right_visual_out),
        Some(tx_atom_event),
    );

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "WaveRecordGUI",
        options,
        Box::new(|_cc| Box::new(wave_record_gui)),
    );
}
