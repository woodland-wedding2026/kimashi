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
        let pixel_size = 5.0;
        let (response, painter) = ui.allocate_painter(
            vec2(
                self.canvas_width as f32 * pixel_size,
                self.canvas_height as f32 * pixel_size,
            ),
            Sense::drag(),
        );

        if let Some(pos) = response.interact_pointer_pos() {
            let x = ((pos.x - response.rect.left()) / pixel_size) as usize;
            let y = ((pos.y - response.rect.top()) / pixel_size) as usize;

            for dy in 0..self.brush_size {
                for dx in 0..self.brush_size {
                    let xi = x.saturating_add(dx);
                    let yi = y.saturating_add(dy);
                    if xi < self.canvas_width && yi < self.canvas_height {
                        self.pixels[yi][xi] = Some(self.brush_color);
                    }
                }
            }
        }

        for (y, row) in self.pixels.iter().enumerate() {
            for (x, &color) in row.iter().enumerate() {
                if let Some(c) = color {
                    let rect = Rect::from_min_size(
                        response.rect.min + vec2(x as f32 * pixel_size, y as f32 * pixel_size),
                        vec2(pixel_size, pixel_size),
                    );
                    painter.rect_filled(rect, 0.0, c);
                }
            }
        }

        response
    }

    fn ui_stroke_canvas(&mut self, ui: &mut Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        let to_screen = egui::emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        let current_stroke = Stroke {
            width: self.stroke_width,
            color: self.brush_color,
        };

        if self.lines.is_empty() {
            self.lines.push((vec![], current_stroke));
        }

        let (current_line, stroke) = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push((vec![], current_stroke));
            response.mark_changed();
        }

        let shapes = self
            .lines
            .iter()
            .filter(|(line, _)| line.len() >= 2)
            .map(|(line, stroke)| {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                egui::Shape::line(points, *stroke)
            });

        painter.extend(shapes);

        response
    }

    pub fn save_to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn load_from_json(&mut self, json: &str) {
        if let Ok(loaded) = serde_json::from_str::<PaintingApp>(json) {
            *self = loaded;
        }
    }
}
