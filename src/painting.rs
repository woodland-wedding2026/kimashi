use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui};
use egui::emath;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Pixel,
    Stroke,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Stroke
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct PaintingApp {
    // Pixel painting storage: 2D grid of optional colors
    pub pixels: Vec<Vec<Option<Color32>>>,
    pub canvas_width: usize,
    pub canvas_height: usize,

    // Stroke painting storage: lines of points with stroke style
    pub lines: Vec<(Vec<Pos2>, Stroke)>,

    // Current brush settings
    pub brush_color: Color32,
    pub brush_size: usize,    // For pixel mode: brush is square of brush_size x brush_size pixels
    pub stroke_width: f32,    // For stroke mode

    // Mode selector
    pub mode: Mode,

    // Saved JSON state for pixels (optional)
    pub saved_json: Option<String>,
}

impl PaintingApp {
    pub fn new(canvas_width: usize, canvas_height: usize) -> Self {
        Self {
            pixels: vec![vec![None; canvas_width]; canvas_height],
            canvas_width,
            canvas_height,
            brush_color: Color32::YELLOW,
            brush_size: 1,
            stroke_width: 2.0,
            mode: Mode::Stroke,
            lines: Vec::new(),
            saved_json: None,
        }
    }

    pub fn ui_control(&mut self, ui: &mut Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label("Mode:");
            ui.radio_value(&mut self.mode, Mode::Stroke, "Stroke");
            ui.radio_value(&mut self.mode, Mode::Pixel, "Pixel");
            ui.separator();

            ui.label("Color:");
            ui.color_edit_button_srgba(&mut self.brush_color);

            ui.separator();

            if self.mode == Mode::Stroke {
                ui.label("Stroke width:");
                ui.add(egui::Slider::new(&mut self.stroke_width, 1.0..=10.0).clamp_to_range(true));
            } else {
                ui.label("Brush size:");
                ui.add(egui::Slider::new(&mut self.brush_size, 1..=10));
            }

            ui.separator();

            if ui.button("Clear Painting").clicked() {
                self.lines.clear();
                for row in &mut self.pixels {
                    for p in row.iter_mut() {
                        *p = None;
                    }
                }
            }

            if ui.button("Save Pixels JSON").clicked() {
                self.saved_json = Some(self.save_pixels_to_json());
            }

            if ui.button("Load Pixels JSON").clicked() {
                if let Some(json) = &self.saved_json {
                    self.load_pixels_from_json(json);
                }
            }
        }).response
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        // Allocate painter for the entire canvas area
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        // Handle input
        if let Some(pos) = response.interact_pointer_pos() {
            match self.mode {
                Mode::Pixel => {
                    // Draw pixels at pointer
                    let pixel_size = 5.0;
                    let x = ((pos.x - response.rect.left()) / pixel_size) as usize;
                    let y = ((pos.y - response.rect.top()) / pixel_size) as usize;

                    for dy in 0..self.brush_size {
                        for dx in 0..self.brush_size {
                            let xi = x.saturating_add(dx);
                            let yi = y.saturating_add(dy);
                            if xi < self.canvas_width && yi < self.canvas_height {
                                self.pixels[yi][xi] = Some(self.brush_color);
                                response.mark_changed();
                            }
                        }
                    }
                }
                Mode::Stroke => {
                    let to_screen = emath::RectTransform::from_to(
                        Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
                        response.rect,
                    );
                    let from_screen = to_screen.inverse();

                    if self.lines.is_empty() {
                        self.lines.push((vec![], Stroke {
                            width: self.stroke_width,
                            color: self.brush_color,
                        }));
                    }

                    let (current_line, current_stroke) = self.lines.last_mut().unwrap();

                    // If brush color or width changed, update current stroke style
                    if current_stroke.color != self.brush_color || (current_stroke.width - self.stroke_width).abs() > f32::EPSILON {
                        *current_stroke = Stroke {
                            width: self.stroke_width,
                            color: self.brush_color,
                        };
                    }

                    let canvas_pos = from_screen * pos;

                    if current_line.last() != Some(&canvas_pos) {
                        current_line.push(canvas_pos);
                        response.mark_changed();
                    }
                }
            }
        } else if self.mode == Mode::Stroke {
            // Finish current stroke line if pointer released
            if let Some((current_line, _)) = self.lines.last() {
                if !current_line.is_empty() {
                    self.lines.push((vec![], Stroke {
                        width: self.stroke_width,
                        color: self.brush_color,
                    }));
                    response.mark_changed();
                }
            }
        }

        // Render pixels
        let pixel_size = 5.0;
        for (y, row) in self.pixels.iter().enumerate() {
            for (x, &color) in row.iter().enumerate() {
                if let Some(c) = color {
                    let rect = Rect::from_min_size(
                        response.rect.min + egui::vec2(x as f32 * pixel_size, y as f32 * pixel_size),
                        egui::vec2(pixel_size, pixel_size),
                    );
                    painter.rect_filled(rect, 0.0, c);
                }
            }
        }

        // Render strokes
        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );

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

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.ui_control(ui); // top controls: mode, color, sliders, clear, save/load

            ui.add_space(4.0);

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.ui_content(ui);
            });
        });
    }

    // Serialize pixels grid to JSON string
    pub fn save_pixels_to_json(&self) -> String {
        serde_json::to_string(&self.pixels).unwrap_or_else(|_| String::new())
    }

    // Load pixels grid from JSON string
    pub fn load_pixels_from_json(&mut self, json: &str) {
        if let Ok(loaded) = serde_json::from_str::<Vec<Vec<Option<Color32>>>>(json) {
            // Validate size before overwriting
            if loaded.len() == self.canvas_height && loaded[0].len() == self.canvas_width {
                self.pixels = loaded;
            }
        }
    }
}
