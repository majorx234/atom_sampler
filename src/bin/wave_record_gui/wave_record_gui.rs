use atom_sampler_lib::ui::elements::{pad_button, pad_button_ui, DebugConsole, WavePlotter};
use eframe::egui::{self, menu, Button, Context, PointerButton, ViewportCommand, Widget};

pub struct WaveRecordGUI {
    wave_loaded: bool,
    pub console: DebugConsole,
    wave_data: Option<Vec<f32>>,
    wave_plotter: Option<WavePlotter>,
    pub wave_pos: Option<usize>,
    pub pad_button_is_pressed: bool,
}

impl Default for WaveRecordGUI {
    fn default() -> Self {
        Self {
            wave_loaded: false,
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
            wave_data: Some(Vec::new()),
            wave_plotter: None,
            wave_pos: None,
            pad_button_is_pressed: false,
        }
    }
}
impl eframe::App for WaveRecordGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("WaveRecordGui");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                let mut _dropped_files = Vec::new();
                let _ = pad_button_ui(
                    ui,
                    &mut self.wave_loaded,
                    &mut _dropped_files,
                    &mut self.pad_button_is_pressed,
                );
                if let Some(ref mut wave_pos) = self.wave_pos {
                    if self.pad_button_is_pressed {
                        *wave_pos += 1000;
                        let max_len = self.wave_data.as_ref().unwrap().len();
                        *wave_pos = (*wave_pos).min(max_len);
                    } else {
                        *wave_pos = 0;
                    }
                }
            });
        });
        egui::TopBottomPanel::bottom("console").show(ctx, |ui| {
            self.console.debug_console_ui(ui);
        });
        //    ctx.request_repaint();
    }
}
