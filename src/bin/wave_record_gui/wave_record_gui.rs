use atom_sampler_lib::ui::elements::{pad_button, pad_button_ui, DebugConsole, WavePlotter};
use eframe::egui::{self, menu, Button, Context, PointerButton, ViewportCommand, Widget};
use ringbuf::{traits::Consumer, HeapCons};

pub struct WaveRecordGUI {
    wave_loaded: bool,
    pub console: DebugConsole,
    wave_data: Option<Vec<f32>>,
    wave_plotter_left: Option<WavePlotter>,
    wave_plotter_right: Option<WavePlotter>,
    pub wave_pos: Option<usize>,
    pub pad_button_is_pressed: bool,
    ringbuffer_left_visual_in_opt: Option<HeapCons<(f32, f32)>>,
    ringbuffer_right_visual_in_opt: Option<HeapCons<(f32, f32)>>,
}

impl WaveRecordGUI {
    pub fn new(
        ringbuffer_left_visual_in_opt: Option<HeapCons<(f32, f32)>>,
        ringbuffer_right_visual_in_opt: Option<HeapCons<(f32, f32)>>,
    ) -> Self {
        WaveRecordGUI {
            wave_loaded: false,
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
            wave_data: Some(Vec::new()),
            wave_plotter_left: Some(WavePlotter::new(20.0, 4.0)),
            wave_plotter_right: Some(WavePlotter::new(20.0, 4.0)),
            wave_pos: None,
            pad_button_is_pressed: false,
            ringbuffer_left_visual_in_opt,
            ringbuffer_right_visual_in_opt,
        }
    }
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
            wave_plotter_left: None,
            wave_plotter_right: None,
            wave_pos: None,
            pad_button_is_pressed: false,
            ringbuffer_left_visual_in_opt: None,
            ringbuffer_right_visual_in_opt: None,
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
                // recording:
                if self.pad_button_is_pressed {
                    if let (
                        Some(mut ringbuffer_left_visual_in),
                        Some(mut ringbuffer_right_visual_in),
                    ) = (
                        self.ringbuffer_left_visual_in_opt.take(),
                        self.ringbuffer_right_visual_in_opt.take(),
                    ) {
                        if let Some(mut wave_plotter_left) = self.wave_plotter_left.take() {
                            let wave_limits_left = ringbuffer_left_visual_in.pop_iter();
                            wave_plotter_left.extend_limits(wave_limits_left);
                        }
                        if let Some(mut wave_plotter_right) = self.wave_plotter_right.take() {
                            let wave_limits_right = ringbuffer_right_visual_in.pop_iter();
                            wave_plotter_right.extend_limits(wave_limits_right);
                        }
                    }
                }
                if let Some(ref mut wave_pos) = self.wave_pos {
                    if self.pad_button_is_pressed {
                        *wave_pos += 1000;
                        let max_len = self.wave_data.as_ref().unwrap().len();
                        *wave_pos = (*wave_pos).min(max_len);
                    } else {
                        *wave_pos = 0;
                    }
                }
                if let Some(ref wave_plotter_left) = self.wave_plotter_left {
                    if let Some(wave_pos) = self.wave_pos {
                        wave_plotter_left.wave_plot_ui(ui, 100, wave_pos);
                    } else {
                        wave_plotter_left.wave_plot_ui(ui, 100, 0);
                    }
                }
                if let Some(ref wave_plotter_right) = self.wave_plotter_right {
                    if let Some(wave_pos) = self.wave_pos {
                        wave_plotter_right.wave_plot_ui(ui, 100, wave_pos);
                    } else {
                        wave_plotter_right.wave_plot_ui(ui, 100, 0);
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
