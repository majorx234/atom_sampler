use std::str::FromStr;

use atom_sampler_lib::ui::elements::DebugConsole;
use eframe::{self, egui::ViewportBuilder};
mod mockup;
use mockup::MockupGUI;

fn main() {
    let msgs: Vec<String> = Vec::new();
    let n_items = msgs.len();
    let mockup_gui = MockupGUI {
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
        Box::new(|_cc| Box::new(mockup_gui)),
    );
}
