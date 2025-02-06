use atom_sampler_lib::ui::elements::{pad_button, pad_button_ui, DebugConsole};
use eframe::egui::{self, menu, Button, Context, PointerButton, ViewportCommand, Widget};
use std::path::PathBuf;

pub struct WaveLoadGUI {
    pub wave_loaded: bool,
    pub console: DebugConsole,
    picked_file: Option<PathBuf>,
}

impl Default for WaveLoadGUI {
    fn default() -> Self {
        Self {
            wave_loaded: false,
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
            picked_file: None,
        }
    }
}
impl eframe::App for WaveLoadGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("WaveLoadGui");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.console.debug_console_ui(ui);
        });
    }
}
