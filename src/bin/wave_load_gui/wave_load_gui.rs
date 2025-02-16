use atom_sampler_lib::ui::elements::{pad_button, pad_button_ui, DebugConsole, WavePlotter};
use eframe::egui::{self, menu, Button, Context, PointerButton, ViewportCommand, Widget};
use std::path::PathBuf;

fn read_wav_file(file_path: &PathBuf) -> Result<Vec<f32>, hound::Error> {
    let mut reader = hound::WavReader::open(file_path)?;
    let spec = reader.spec();
    let max_val = 2.0f32.powf(spec.bits_per_sample as f32) / 2.0f32;

    let wave_data: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().flatten().collect(),
        hound::SampleFormat::Int => reader
            .samples::<i32>()
            .flatten()
            .map(|x| x as f32 / max_val)
            .collect(),
    };

    Ok(wave_data)
}
pub struct WaveLoadGUI {
    pub file_name_str: String,
    pub wave_loaded: bool,
    pub console: DebugConsole,
    picked_file: Option<PathBuf>,
    wave_data: Option<Vec<f32>>,
    wave_plotter: Option<WavePlotter>,
    pub wave_pos: Option<usize>,
    pub pad_button_is_pressed: bool,
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
            wave_plotter: None,
            wave_pos: None,
            pad_button_is_pressed: false,
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
                self.wave_loaded = false;
                self.picked_file = Some(PathBuf::from(&self.file_name_str));
                if let Some(ref picked_file) = self.picked_file {
                    self.console
                        .add_entry(picked_file.to_str().get_or_insert("none").to_string());
                }
            }
            if ui.button("load wave").clicked() {
                if let Some(ref picked_file) = self.picked_file {
                    if picked_file.is_file() {
                        if let Ok(samples_vec) = read_wav_file(picked_file) {
                            let num_samples = samples_vec.len();

                            let mut wave_plotter = WavePlotter::new(20.0, 4.0);
                            wave_plotter.load_wave(&samples_vec);
                            self.wave_plotter = Some(wave_plotter);
                            self.wave_data = Some(samples_vec);
                            self.wave_loaded = true;
                            self.wave_pos = Some(0);
                            self.console
                                .add_entry(format!("wave loaded: {} samples", num_samples));
                        }
                    }
                }
            }
            let mut dropped_files: Vec<egui::DroppedFile> = Vec::new();
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                let _ = pad_button_ui(
                    ui,
                    &mut self.wave_loaded,
                    &mut dropped_files,
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
                let mut dropped_file_paths = Vec::new();
                if !dropped_files.is_empty() {
                    self.console.add_entry("droped file:".to_string());
                    for (idx, file) in dropped_files.iter().enumerate() {
                        let filepath = file.path.clone().expect("no file path");
                        let file_msg = format!(
                            "file {}: {} - Mime: {}, Filepath: {}",
                            idx,
                            file.name,
                            file.mime,
                            filepath.as_path().display()
                        );
                        self.console.add_entry(file_msg);
                        dropped_file_paths.push(filepath);
                    }
                }
                if !dropped_file_paths.is_empty() {
                    self.picked_file = if dropped_file_paths[0].is_file() {
                        Some(dropped_file_paths[0].clone())
                    } else {
                        self.picked_file.take()
                    };
                    if let Some(ref picked_file) = self.picked_file {
                        if picked_file.is_file() {
                            if let Ok(samples_vec) = read_wav_file(picked_file) {
                                let num_samples = samples_vec.len();

                                let mut wave_plotter = WavePlotter::new(20.0, 4.0);
                                wave_plotter.load_wave(&samples_vec);
                                self.wave_plotter = Some(wave_plotter);
                                self.wave_data = Some(samples_vec);
                                self.wave_loaded = true;
                                self.wave_pos = Some(0);
                                self.console
                                    .add_entry(format!("wave loaded: {} samples", num_samples));
                            }
                        }
                    }
                }
            });
            if let Some(ref wave_plotter) = self.wave_plotter {
                if let Some(wave_pos) = self.wave_pos {
                    wave_plotter.wave_plot_ui(ui, 100, wave_pos);
                } else {
                    wave_plotter.wave_plot_ui(ui, 100, 0);
                }
            }
        });
        egui::TopBottomPanel::bottom("console").show(ctx, |ui| {
            self.console.debug_console_ui(ui);
        });
    }
}
