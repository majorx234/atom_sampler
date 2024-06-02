use eframe::{egui, epaint::Stroke};

pub fn pad_button(ui: &mut egui::Ui, status_wave_loaded: &bool) -> egui::Response {
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
