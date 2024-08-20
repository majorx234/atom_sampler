use atom_sampler_lib::ui::elements::DebugConsole;
use eframe::{self, egui::ViewportBuilder};
mod atom_sampler_app;
use atom_sampler_app::AtomSamplerApp;

fn main() {
    println!("atom sampler WIP!");
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
