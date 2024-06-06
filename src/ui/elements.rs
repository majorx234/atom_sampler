use eframe::{
    egui::{self, ScrollArea},
    epaint::Stroke,
};

pub fn pad_button_ui(ui: &mut egui::Ui, status_wave_loaded: &bool) -> egui::Response {
    let width = 4.0;
    let height = 4.0;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(width, height);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    // TODO: check if clicked

    if ui.is_rect_visible(response.rect) {
        let visuals = ui.style().visuals.clone();
        let rounding = rect.height() / 2.0;
        ui.painter()
            .rect(rect, rounding, visuals.extreme_bg_color, Stroke::NONE);
    }
    response
}
pub fn pad_button(status_wave_loaded: &bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| pad_button_ui(ui, status_wave_loaded)
}

pub struct DebugConsole {
    pub n_items: usize,
    pub msgs: Vec<String>,
}

impl DebugConsole {
    pub fn debug_console_ui(&self, ui: &mut egui::Ui) {
        let n_items = self.msgs.len();
        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        if n_items > 0 {
            ScrollArea::vertical()
                .stick_to_bottom(true)
                .min_scrolled_height(600.0)
                .max_height(600.0)
                .min_scrolled_width(500.0)
                .max_width(500.0)
                .show_rows(ui, row_height, n_items, |ui, row_range| {
                    for row in row_range {
                        let text = format!("{} {}", row, self.msgs[row]);
                        ui.label(text);
                    }
                });
        }
    }
}
