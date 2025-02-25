use eframe::{self, egui::ViewportBuilder};
mod wave_record_gui;
use atom_sampler_lib::{
    atom_event::AtomEvent, jackmidi::MidiMsgGeneric, jackprocess::start_jack_thread,
    wave_manager::start_wave_manager,
};
use bus::Bus;
use ringbuf::{traits::Split, HeapRb};
use wave_record_gui::WaveRecordGUI;

fn main() {
    let ringbuffer_left = HeapRb::<f32>::new(192000);
    let ringbuffer_right = HeapRb::<f32>::new(192000);

    let ringbuffer_left_play = HeapRb::<f32>::new(192000);
    let ringbuffer_right_play = HeapRb::<f32>::new(192000);

    let ringbuffer_left_visual = HeapRb::<(f32, f32)>::new(375);
    let ringbuffer_right_visual = HeapRb::<(f32, f32)>::new(375);

    let (ringbuffer_left_in, ringbuffer_left_out) = ringbuffer_left.split();
    let (ringbuffer_right_in, ringbuffer_right_out) = ringbuffer_right.split();

    let (ringbuffer_left_play_in, ringbuffer_left_play_out) = ringbuffer_left_play.split();
    let (ringbuffer_right_play_in, ringbuffer_right_play_out) = ringbuffer_right_play.split();

    let (ringbuffer_left_visual_in, ringbuffer_left_visual_out) = ringbuffer_left_visual.split();
    let (ringbuffer_right_visual_in, ringbuffer_right_visual_out) = ringbuffer_right_visual.split();

    let mut tx_close = Bus::<bool>::new(1);
    let rx1_close = tx_close.add_rx();
    let rx2_close = tx_close.add_rx();

    let mut tx_atom_event = Bus::<AtomEvent>::new(1);
    let rx1_atom_event = tx_atom_event.add_rx();
    let rx2_atom_event = tx_atom_event.add_rx();

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
        rx1_atom_event,
    );

    start_wave_manager(
        rx2_close,
        ringbuffer_left_out,
        ringbuffer_right_out,
        ringbuffer_left_play_in,
        ringbuffer_right_play_in,
        ringbuffer_left_visual_in,
        ringbuffer_right_visual_in,
        rx2_atom_event,
    );
    let wave_record_gui = WaveRecordGUI::new(
        Some(ringbuffer_left_visual_out),
        Some(ringbuffer_right_visual_out),
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
