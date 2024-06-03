use eframe::{self, egui::ViewportBuilder};
mod mockup;
use mockup::MockupGUI;

fn main() {
    let mockup_gui = MockupGUI::default();
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
