use atom_sampler_lib::ui::elements::{pad_button, pad_button_ui, DebugConsole};
use eframe::egui::{self, menu, Button, Context, PointerButton, ViewportCommand, Widget};
use hound;
use std::path::PathBuf;

fn read_wav_file(file_path: &PathBuf) -> Result<Vec<f32>, hound::Error> {
    let mut reader = hound::WavReader::open(file_path).unwrap();
    let spec = reader.spec();

    let wave_data: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().flatten().collect(),
        hound::SampleFormat::Int => reader
            .samples::<i16>()
            .flatten()
            .map(|x| x as f32 / i16::MAX as f32)
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
                            self.wave_data = Some(samples_vec);
                            self.wave_loaded = true;
                            self.console
                                .add_entry(format!("wave loaded: {} samples", num_samples));
                        }
                    }
                }
            }
            self.console.debug_console_ui(ui);
        });
    }
}
