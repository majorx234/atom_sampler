use atom_sampler_lib::{
    atom_event::{AtomEvent, Type},
    ui::elements::{
        pad_button, pad_button_ui, status_button, status_button_ui, DebugConsole, WavePlotter,
    },
};
use bus::Bus;
use eframe::egui::{self, menu, Button, Context, PointerButton, ViewportCommand, Widget};
use ringbuf::{traits::Consumer, HeapCons};
use std::thread::JoinHandle;

pub struct WaveSamplerWidgetGUI {
    wave_loaded: bool,
    record_mode: bool,
    pub console: DebugConsole,
    wave_data: Option<Vec<f32>>,
    wave_plotter_left: Option<WavePlotter>,
    wave_plotter_right: Option<WavePlotter>,
    pub wave_pos: Option<usize>,
    pub pad_button_is_pressed: bool,
    pub pad_button_was_pressed: bool,
    ringbuffer_left_visual_in_opt: Option<HeapCons<(f32, f32)>>,
    ringbuffer_right_visual_in_opt: Option<HeapCons<(f32, f32)>>,
    tx_close_event: Option<Bus<bool>>,
    tx_atom_event: Option<Bus<AtomEvent>>,
    jack_join_handle: Option<JoinHandle<()>>,
    wave_manager_join_handle: Option<JoinHandle<()>>,
}

impl WaveSamplerWidgetGUI {
    pub fn new(
        ringbuffer_left_visual_in_opt: Option<HeapCons<(f32, f32)>>,
        ringbuffer_right_visual_in_opt: Option<HeapCons<(f32, f32)>>,
        tx_close_event: Option<Bus<bool>>,
        tx_atom_event: Option<Bus<AtomEvent>>,
        jack_join_handle: Option<JoinHandle<()>>,
        wave_manager_join_handle: Option<JoinHandle<()>>,
    ) -> Self {
        WaveSamplerWidgetGUI {
            wave_loaded: false,
            record_mode: false,
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
            wave_data: Some(Vec::new()),
            wave_plotter_left: Some(WavePlotter::new(20.0, 4.0)),
            wave_plotter_right: Some(WavePlotter::new(20.0, 4.0)),
            wave_pos: None,
            pad_button_is_pressed: false,
            pad_button_was_pressed: false,
            ringbuffer_left_visual_in_opt,
            ringbuffer_right_visual_in_opt,
            tx_close_event,
            tx_atom_event,
            jack_join_handle,
            wave_manager_join_handle,
        }
    }
}

impl Default for WaveSamplerWidgetGUI {
    fn default() -> Self {
        Self {
            wave_loaded: false,
            record_mode: false,
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
            wave_data: Some(Vec::new()),
            wave_plotter_left: None,
            wave_plotter_right: None,
            wave_pos: None,
            pad_button_is_pressed: false,
            pad_button_was_pressed: false,
            ringbuffer_left_visual_in_opt: None,
            ringbuffer_right_visual_in_opt: None,
            tx_close_event: None,
            tx_atom_event: None,
            jack_join_handle: None,
            wave_manager_join_handle: None,
        }
    }
}

impl eframe::App for WaveSamplerWidgetGUI {
    fn on_exit(&mut self, _ctx: Option<&eframe::glow::Context>) {
        println!("close WaveSamplerWidgetGUI");
        self.tx_close_event
            .as_mut()
            .and_then(|tx| tx.try_broadcast(false).ok());
        if let Some(jack_join_handle) = self.jack_join_handle.take() {
            let _ = jack_join_handle.join();
        }
        if let Some(wave_manager_join_handle) = self.wave_manager_join_handle.take() {
            let _ = wave_manager_join_handle.join();
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("WaveSamplerWidgetGUI");
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
                    if self.record_mode {
                        if let (
                            Some(mut ringbuffer_left_visual_in),
                            Some(mut ringbuffer_right_visual_in),
                        ) = (
                            self.ringbuffer_left_visual_in_opt.take(),
                            self.ringbuffer_right_visual_in_opt.take(),
                        ) {
                            if let Some(ref mut wave_plotter_left) = self.wave_plotter_left {
                                wave_plotter_left.append_limits(
                                    &mut ringbuffer_left_visual_in
                                        .pop_iter()
                                        .collect::<Vec<(f32, f32)>>(),
                                );
                                self.ringbuffer_left_visual_in_opt =
                                    Some(ringbuffer_left_visual_in);
                            }
                            if let Some(ref mut wave_plotter_right) = self.wave_plotter_right {
                                wave_plotter_right.append_limits(
                                    &mut ringbuffer_right_visual_in
                                        .pop_iter()
                                        .collect::<Vec<(f32, f32)>>(),
                                );
                                self.ringbuffer_right_visual_in_opt =
                                    Some(ringbuffer_right_visual_in);
                            }
                        }
                    } else {
                        // playback mode
                    }
                }
                if !self.pad_button_was_pressed && self.pad_button_is_pressed {
                    if self.record_mode {
                        if let Some(ref mut tx_atom_event) = self.tx_atom_event {
                            if let Some(ref mut wav_plotter) = self.wave_plotter_left {
                                wav_plotter.reset_wave();
                            }
                            if let Some(ref mut wav_plotter) = self.wave_plotter_right {
                                wav_plotter.reset_wave();
                            }
                            let is_sent = tx_atom_event.try_broadcast(AtomEvent {
                                event_type: Type::Recording(true),
                                start: true,
                            });
                            self.console
                                .add_entry(format!("start recording {:?}", is_sent));
                        }
                    } else {
                        // playback mode
                        if let Some(ref mut tx_atom_event) = self.tx_atom_event {
                            let is_sent = tx_atom_event.try_broadcast(AtomEvent {
                                event_type: Type::Playback(true),
                                start: true,
                            });

                            self.console
                                .add_entry(format!("start playback {:?}", is_sent));
                        }
                    }
                }
                if self.pad_button_was_pressed && !self.pad_button_is_pressed {
                    if self.record_mode {
                        if let Some(ref mut tx_atom_event) = self.tx_atom_event {
                            let is_sent = tx_atom_event.try_broadcast(AtomEvent {
                                event_type: Type::Recording(false),
                                start: false,
                            });
                            self.console
                                .add_entry(format!("stop recording {:?}", is_sent));
                        }
                    } else {
                        // playback mode
                        if let Some(ref mut tx_atom_event) = self.tx_atom_event {
                            let is_sent = tx_atom_event.try_broadcast(AtomEvent {
                                event_type: Type::Playback(false),
                                start: false,
                            });
                            self.console
                                .add_entry(format!("stop recording {:?}", is_sent));
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
                self.pad_button_was_pressed = self.pad_button_is_pressed;
            });
            let _ = status_button_ui(ui, &mut self.record_mode);
        });
        egui::TopBottomPanel::bottom("console").show(ctx, |ui| {
            self.console.debug_console_ui(ui);
        });
        if self.pad_button_is_pressed {
            ctx.request_repaint();
        }
        //    ctx.request_repaint();
    }
}
