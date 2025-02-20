use eframe::{self, egui::ViewportBuilder};
mod wave_record_gui;
use wave_record_gui::WaveRecordGUI;

fn main() {
    let wave_record_gui = WaveRecordGUI::default();
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
