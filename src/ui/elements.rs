use eframe::{
    egui::{self, Color32, PointerButton, Pos2, Rangef, ScrollArea},
    epaint::Stroke,
};

pub struct WavePlotter {
    inited: bool,
    wave_loaded: bool,
    width: f32,
    height: f32,
    limits: Vec<(f32, f32)>,
    dpi: usize,
    wave_length: usize,
    short_wave: bool,
}

impl WavePlotter {
    pub fn new(width: f32, height: f32) -> Self {
        WavePlotter {
            inited: true,
            wave_loaded: false,
            width,
            height,
            limits: Vec::new(),
            dpi: 100,
            wave_length: 0,
            short_wave: true,
        }
    }
    pub fn wave_plot_ui(&self, ui: &mut egui::Ui, dpi: usize, pos: usize) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(self.width, self.height);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        let visuals = ui.style().visuals.clone();
        ui.painter().rect(
            rect,
            0.0,
            visuals.panel_fill,
            Stroke::new(1.0, Color32::DARK_BLUE),
        );
        let x_max = self.limits.len();
        for (x_idx, (y_low, y_high)) in self.limits.iter().enumerate() {
            let x = x_idx as f32 * rect.width() / x_max as f32 + rect.min.x;
            let y_low_in_rect = (*y_low + 1.0) * rect.height() / 2.0 + rect.min.y;
            let y_high_in_rect = (*y_high + 1.0) * rect.height() / 2.0 + rect.min.y;
            ui.painter().vline(
                x,
                Rangef::new(y_low_in_rect, y_high_in_rect),
                Stroke::new(1.0, Color32::GREEN),
            );

            let pos_in_rect = if !self.short_wave {
                let pos_in_limits =
                    pos as f32 * (self.limits.len() as f32) / (self.wave_length as f32);
                pos_in_limits * rect.width() / x_max as f32 + rect.min.x
            } else {
                pos as f32 * rect.width() / x_max as f32 + rect.min.x
            };
            ui.painter().vline(
                pos_in_rect,
                Rangef::new(rect.min.y, rect.max.y),
                Stroke::new(1.0, Color32::YELLOW),
            );
        }
        response
    }
    pub fn load_wave(&mut self, wave: &[f32]) {
        self.wave_length = wave.len();
        let segments: usize = (self.width * self.dpi as f32) as usize;
        // TODO check if segments are more than  wave length
        let mut limits: Vec<(f32, f32)> = vec![(0.0, 0.0); segments];
        if segments < wave.len() {
            let sample_per_segment = wave.len() / segments;
            // ToDo handle residium, fill rest up with zeros
            for segment in 0..segments {
                for sample in 0..sample_per_segment {
                    let j = segment * sample_per_segment + sample;
                    if wave[j] < limits[segment].0 {
                        limits[segment].0 = wave[j];
                    } else if wave[j] > limits[segment].1 {
                        limits[segment].1 = wave[j];
                    }
                }
            }
            self.short_wave = false;
        } else {
            for (idx, (limit, wave)) in limits.iter_mut().zip(wave.iter()).enumerate() {
                if *wave < 0.0 {
                    limit.0 = *wave;
                    limit.1 = 0.0;
                } else {
                    limit.0 = 0.0;
                    limit.1 = *wave;
                }
            }
            self.short_wave = true
        }
        self.limits = limits;
        self.wave_loaded = true;
    }
    pub fn append_limits(&mut self, limits_slice: &mut Vec<(f32, f32)>) {
        self.limits.append(limits_slice);
    }
    pub fn extend_limits(&mut self, limits_iter: impl Iterator<Item = (f32, f32)>) {
        self.limits.extend(limits_iter);
    }
    pub fn reset_wave(&mut self) {
        self.limits = Vec::new();
        self.wave_loaded = false;
    }
}

pub fn pad_button_ui(
    ui: &mut egui::Ui,
    status_wave_loaded: &mut bool,
    dropped_files: &mut Vec<egui::DroppedFile>,
    is_pressed: &mut bool,
) -> egui::Response {
    let mut clicked = false;
    let width = 4.0;
    let height = 4.0;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(width, height);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    let dont_check_release_in_rect: bool = true;
    if response.clicked() {
        clicked = true;
        response.mark_changed(); // report back that the value changed
                                 // TODO animation when clicked
    }

    ui.input(|input| {
        if input.pointer.button_pressed(PointerButton::Primary) {
            if response.rect.contains(
                *input
                    .pointer
                    .press_origin()
                    .get_or_insert(Pos2::new(-1.0, -1.0)),
            ) {
                *is_pressed = true;
            }
        } else if input.pointer.button_released(PointerButton::Primary)
            && input.pointer.interact_pos().is_some()
            && (response
                .rect
                .contains(input.pointer.interact_pos().unwrap())
                || dont_check_release_in_rect)
        {
            *is_pressed = false;
        }
    });

    if ui.is_rect_visible(response.rect) {
        let visuals = ui.style().visuals.clone();
        let rounding = rect.height() / 8.0;
        let color = if *is_pressed {
            visuals.warn_fg_color
        } else {
            visuals.extreme_bg_color
        };
        ui.painter().rect(rect, rounding, color, Stroke::NONE);
    }
    ui.ctx().input(|i| {
        if !i.raw.dropped_files.is_empty() && rect.contains(i.pointer.interact_pos().unwrap()) {
            dropped_files.append(&mut i.raw.dropped_files.clone());
        }
    });
    response.interact(egui::Sense {
        click: true,
        drag: true,
        focusable: true,
    });
    response
}

pub fn pad_button_ui2(
    ui: &mut egui::Ui,
    status_wave_loaded: &mut bool,
    dropped_files: &mut Vec<egui::DroppedFile>,
) -> egui::Response {
    let width = 4.0;
    let height = 4.0;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(width, height);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    response
}

pub fn pad_button<'a>(
    status_wave_loaded: &'a mut bool,
    dropped_files: &'a mut Vec<egui::DroppedFile>,
    is_pressed: &'a mut bool,
) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| pad_button_ui(ui, status_wave_loaded, dropped_files, is_pressed)
}

pub struct DebugConsole {
    pub n_items: usize,
    pub msgs: Vec<String>,
}

impl DebugConsole {
    pub fn add_entry(&mut self, msg: String) {
        self.msgs.push(msg);
        self.n_items = self.msgs.len();
    }
    pub fn debug_console_ui(&self, ui: &mut egui::Ui) {
        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        if self.n_items > 0 {
            ScrollArea::vertical()
                .stick_to_bottom(true)
                .min_scrolled_height(60.0)
                .max_height(240.0)
                .min_scrolled_width(500.0)
                .max_width(500.0)
                .show_rows(ui, row_height, self.n_items, |ui, row_range| {
                    for row in row_range {
                        let text = format!("{} {}", row, self.msgs[row]);
                        ui.label(text);
                    }
                });
        }
    }
}

pub fn status_button_ui(ui: &mut egui::Ui, status: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *status = !*status;
        response.mark_changed(); // report back that the value changed
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *status, ""));
    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *status);
        let visuals = ui.style().interact_selectable(&response, *status);

        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }
    response
}

pub fn status_button(status: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| status_button_ui(ui, status)
}
