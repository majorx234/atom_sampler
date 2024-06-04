use atom_sampler_lib::ui::elements::pad_button;
use eframe::egui::{self, ViewportCommand};

pub struct DebugConsole {
    pub n_items: usize,
    pub msgs: Vec<String>,
}

pub struct MockupGUI {
    wave_loaded: bool,
    console: DebugConsole,
}

impl Default for MockupGUI {
    fn default() -> Self {
        Self {
            wave_loaded: false,
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
        }
    }
}

impl eframe::App for MockupGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("MockupGui");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            pad_button(ui, &self.wave_loaded);
        });
    }
}
