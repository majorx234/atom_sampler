use eframe::egui::{self, ViewportCommand};

pub struct MockupGUI {}

impl Default for MockupGUI {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MockupGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
