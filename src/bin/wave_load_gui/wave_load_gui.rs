use atom_sampler_lib::ui::elements::{pad_button, pad_button_ui, DebugConsole};
use eframe::egui::{self, menu, Button, Context, PointerButton, ViewportCommand, Widget};
use hound;
use std::path::PathBuf;

pub struct WaveLoadGUI {
    pub file_name_str: String,
    pub wave_loaded: bool,
    pub console: DebugConsole,
    picked_file: Option<PathBuf>,
    wave_data: Option<Vec<f32>>,
}

impl Default for WaveLoadGUI {
    fn default() -> Self {
        Self {
            file_name_str: String::new(),
            wave_loaded: false,
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
            picked_file: None,
            wave_data: Some(Vec::new()),
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
            ui.label("Filepath:");
            ui.text_edit_singleline(&mut self.file_name_str);
            if ui.button("load file").clicked() {
                self.picked_file = Some(PathBuf::from(&self.file_name_str));
                if let Some(ref picked_file) = self.picked_file {
                    self.console
                        .add_entry(picked_file.to_str().get_or_insert("none").to_string());
                }
            }
            if let Some(ref picked_file) = self.picked_file {
                if picked_file.is_file() {
                    let mut reader = hound::WavReader::open(picked_file).unwrap();
                    self.wave_data = Some(reader.samples::<f32>().flatten().collect());
                }
            }
            self.console.debug_console_ui(ui);
        });
    }
}
