use atom_sampler_lib::ui::elements::DebugConsole;
use eframe::{self, egui::ViewportBuilder};
mod wave_load_gui;
use wave_load_gui::WaveLoadGUI;

fn main() {
    let wave_load_gui = WaveLoadGUI::default();
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "WaveLoadGUI",
        options,
        Box::new(|_cc| Box::new(wave_load_gui)),
    );
}
