use atom_sampler_lib::ui::elements::{pad_button, pad_button_ui, DebugConsole};
use eframe::egui::{self, ViewportCommand, Widget};

pub struct MockupGUI {
    pub wave_loaded: bool,
    pub console: DebugConsole,
}

impl Default for MockupGUI {
    fn default() -> Self {
        Self {
            wave_loaded: false,
            console: DebugConsole {
                n_items: 0,
                msgs: Vec::new(),
            },
        }
    }
}

impl eframe::App for MockupGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut pad_button_clicked = false;
        let mut status_wave_loaded = false;

        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("MockupGui");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let pad_button_clicked_rect = pad_button_ui(ui, &self.wave_loaded)
                .interact(egui::Sense {
                    click: true,
                    drag: true,
                    focusable: true,
                })
                .rect;
            ui.input(|input| {
                if input.pointer.button_pressed(egui::PointerButton::Primary)
                    && pad_button_clicked_rect.contains(input.pointer.press_origin().unwrap())
                {
                    self.console.add_entry("clicked".to_string());
                } else if input.pointer.button_released(egui::PointerButton::Primary) {
                    self.console.add_entry("released".to_string());
                }
            });
            self.console.debug_console_ui(ui);
        });
    }
}
