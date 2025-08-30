use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, vec2};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Mode {
    Pixel,
    Stroke,
}

#[derive(Serialize, Deserialize)]
pub struct PaintingApp {
    // For pixel mode
    pub pixels: Vec<Vec<Option<Color32>>>,
    pub canvas_width: usize,
    pub canvas_height: usize,

    // For stroke mode
    pub lines: Vec<(Vec<Pos2>, Stroke)>,

    // Settings
    pub brush_color: Color32,
    pub brush_size: usize,
    pub stroke_width: f32,
    pub mode: Mode,

    // Optional: JSON string to save/load state
    #[serde(skip)]
    pub saved_json: Option<String>,
}

impl Default for PaintingApp {
    fn default() -> Self {
        let canvas_width = 128;
        let canvas_height = 128;

        Self {
            pixels: vec![vec![None; canvas_width]; canvas_height],
            canvas_width,
            canvas_height,
            lines: vec![],
            brush_color: Color32::YELLOW,
            brush_size: 2,
            stroke_width: 1.0,
            mode: Mode::Pixel,
            saved_json: None,
        }
    }
}

impl PaintingApp {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.ui_control(ui);
            ui.add_space(1.0);
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.ui_content(ui);
            });
        });
    }

    pub fn ui_control(&mut self, ui: &mut Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label("Mode:");
            ui.selectable_value(&mut self.mode, Mode::Pixel, "Pixel");
            ui.selectable_value(&mut self.mode, Mode::Stroke, "Stroke");

            ui.separator();

            ui.color_edit_button_srgba(&mut self.brush_color);

            match self.mode {
                Mode::Pixel => {
                    ui.label("Brush size:");
                    ui.add(egui::Slider::new(&mut self.brush_size, 1..=10));
                }
                Mode::Stroke => {
                    ui.label("Stroke width:");
                    ui.add(egui::Slider::new(&mut self.stroke_width, 0.5..=10.0).text("px"));
                }
            }

            ui.separator();

            if ui.button("Clear").clicked() {
                self.pixels = vec![vec![None; self.canvas_width]; self.canvas_height];
                self.lines.clear();
            }

            if ui.button("Save").clicked() {
                self.saved_json = Some(self.save_to_json());
            }

            if ui.button("Load").clicked() {
                if let Some(json) = self.saved_json.clone() {
                    self.load_from_json(&json);
                }
            }
        })
        .response
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        match self.mode {
            Mode::Pixel => self.ui_pixel_canvas(ui),
            Mode::Stroke => self.ui_stroke_canvas(ui),
        }
    }

    fn ui_pixel_canvas(&mut self, ui: &mut Ui) -> egui::Response {
