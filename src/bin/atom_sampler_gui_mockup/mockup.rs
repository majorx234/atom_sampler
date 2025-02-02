use atom_sampler_lib::ui::elements::{pad_button, pad_button_ui, DebugConsole};
use eframe::egui::{self, PointerButton, ViewportCommand, Widget};

pub struct MockupGUI {
    pub wave_loaded: bool,
    pub pad_button_is_pressed: bool,
    pub console: DebugConsole,
}

impl Default for MockupGUI {
    fn default() -> Self {
        Self {
            wave_loaded: false,
            pad_button_is_pressed: false,
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
            let mut dropped_files: Vec<egui::DroppedFile> = Vec::new();
            let pad_button_clicked_rect = pad_button_ui(
                ui,
                &mut self.wave_loaded,
                &mut dropped_files,
                self.pad_button_is_pressed,
            )
            .interact(egui::Sense {
                click: true,
                drag: true,
                focusable: true,
            })
            .rect;
            ui.input(|input| {
                if input.pointer.button_pressed(PointerButton::Primary)
                    && pad_button_clicked_rect.contains(input.pointer.press_origin().unwrap())
                {
                    self.pad_button_is_pressed = true;
                } else if input.pointer.button_released(PointerButton::Primary) {
                    self.pad_button_is_pressed = false
                }
            });
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
