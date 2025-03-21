
use atom_sampler_lib::{
    atom_event::AtomEvent,
    ui::elements::{pad_button_ui, DebugConsole},
};
use bus::Bus;
use eframe::egui::{self};
use ringbuf::{
    traits::{Consumer, Observer},
    HeapCons,
};
pub struct AtomSamplerApp {
    pub wave_loaded: bool,
    pub pad_button_is_pressed: bool,
    pub console: DebugConsole,
    pub tx_close: Option<Bus<bool>>,
    pub tx_atom_event: Option<Bus<AtomEvent>>,
    pub ringbuffer_left_out: Option<HeapCons<(f32, f32)>>,
    pub ringbuffer_right_out: Option<HeapCons<(f32, f32)>>,
}

impl AtomSamplerApp {
    pub fn new(
        wave_loaded: bool,
        pad_button_is_pressed: bool,
        console: DebugConsole,
        tx_close: Bus<bool>,
        tx_atom_event: Bus<AtomEvent>,
        ringbuffer_left_out: Option<HeapCons<(f32, f32)>>,
        ringbuffer_right_out: Option<HeapCons<(f32, f32)>>,
    ) -> Self {
        Self {
            wave_loaded,
            pad_button_is_pressed,
            console,
            tx_close: Some(tx_close),
            tx_atom_event: Some(tx_atom_event),
            ringbuffer_left_out,
            ringbuffer_right_out,
        }
    }
}

impl Default for AtomSamplerApp {
    fn default() -> Self {
        Self {
            wave_loaded: false,
            pad_button_is_pressed: false,
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
            tx_close: None,
            tx_atom_event: None,
            ringbuffer_left_out: None,
            ringbuffer_right_out: None,
        }
    }
}

impl eframe::App for AtomSamplerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &mut self.ringbuffer_left_out {
            Some(ringbuffer_left_out) => {
                while ringbuffer_left_out.occupied_len() > 512 {
                    let mut values: Vec<f32> = vec![0.0; 512];
                }
            }
            None => (),
        };
        match &mut self.ringbuffer_right_out {
            Some(ringbuffer_right_out) => {
                while ringbuffer_right_out.occupied_len() > 512 {
                    let mut values: Vec<f32> = vec![0.0; 512];
                }
            }
            None => (),
        };
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("MockupGui");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut dropped_files: Vec<egui::DroppedFile> = Vec::new();
            let pad_button_clicked_rect = pad_button_ui(
                ui,
                &mut self.wave_loaded,
                &mut dropped_files,
                &mut self.pad_button_is_pressed,
            )
            .interact(egui::Sense {
                click: true,
                drag: true,
                focusable: true,
            })
            .rect;
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
                }
            }

            ui.input(|input| {
                if input.pointer.button_pressed(egui::PointerButton::Primary)
                    && pad_button_clicked_rect.contains(input.pointer.press_origin().unwrap())
                {
                    self.console.add_entry("clicked".to_string());
                } else if input.pointer.button_released(egui::PointerButton::Primary)
                    && input.pointer.interact_pos().is_some()
                {
                    if pad_button_clicked_rect.contains(input.pointer.interact_pos().unwrap()) {
                        self.console.add_entry("released".to_string());
                    }
                }
            });
            self.console.debug_console_ui(ui);
        });
    }
}
