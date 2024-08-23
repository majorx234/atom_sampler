use std::{mem::MaybeUninit, sync::Arc};

use atom_sampler_lib::{
    atom_event::AtomEvent,
    ui::elements::{pad_button, pad_button_ui, DebugConsole},
};
use bus::Bus;
use crossbeam_channel::Sender;
use eframe::egui::{self, ViewportCommand, Widget};
use ringbuf::{Consumer, SharedRb};

pub struct AtomSamplerApp {
    pub wave_loaded: bool,
    pub console: DebugConsole,
    pub tx_close: Option<Bus<bool>>,
    pub tx_atom_event: Option<Sender<AtomEvent>>,
    pub ringbuffer_left_out:
        Option<Consumer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>>,
    pub ringbuffer_right_out:
        Option<Consumer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>>,
}

impl AtomSamplerApp {
    pub fn new(
        wave_loaded: bool,
        console: DebugConsole,
        tx_close: Bus<bool>,
        tx_atom_event: Sender<AtomEvent>,
        ringbuffer_left_out: Option<
            Consumer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>,
        >,
        ringbuffer_right_out: Option<
            Consumer<f32, Arc<SharedRb<f32, std::vec::Vec<MaybeUninit<f32>>>>>,
        >,
    ) -> Self {
        Self {
            wave_loaded,
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
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("MockupGui");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut dropped_files: Vec<egui::DroppedFile> = Vec::new();
            let pad_button_clicked_rect =
                pad_button_ui(ui, &mut self.wave_loaded, &mut dropped_files)
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
