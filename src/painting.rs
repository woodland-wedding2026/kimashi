use egui::{Color32, Pos2, Rect, Sense, Ui, emath, vec2};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaintingApp {
    pub pixels: Vec<Vec<Option<Color32>>>,
    pub brush_color: Color32,
    pub brush_size: usize,
    pub canvas_width: usize,
    pub canvas_height: usize,
    #[serde(skip)]
    pub saved_json: Option<String>,
}

impl Default for PaintingApp {
    fn default() -> Self {
        let canvas_width = 128;
        let canvas_height = 128;
        Self {
            pixels: vec![vec![None; canvas_width]; canvas_height],
            brush_color: Color32::YELLOW,
            brush_size: 2,
            canvas_width,
            canvas_height,
            saved_json: None,
        }
    }
}

impl PaintingApp {
    pub fn ui_control(&mut self, ui: &mut Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.color_edit_button_srgba(&mut self.brush_color);
            ui.label("Brush size:");
            ui.add(egui::Slider::new(&mut self.brush_size, 1..=10));

            if ui.button("Clear Painting").clicked() {
                for row in &mut self.pixels {
                    for px in row.iter_mut() {
                        *px = None;
                    }
                }
            }

            if ui.button("Save").clicked() {
                self.saved_json = Some(self.save_to_json());
                if let Some(json) = &self.saved_json {
                    println!("Saved JSON:\n{}", json); // for debugging
                }
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
        let pixel_size = 5.0; // visual size of each pixel

        let (response, painter) = ui.allocate_painter(
            vec2(
                self.canvas_width as f32 * pixel_size,
                self.canvas_height as f32 * pixel_size,
            ),
            Sense::drag(),
        );

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let x = ((pointer_pos.x - response.rect.left()) / pixel_size) as usize;
            let y = ((pointer_pos.y - response.rect.top()) / pixel_size) as usize;

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
                if let Some(col) = color {
                    let rect = Rect::from_min_size(
                        response.rect.min + vec2(x as f32 * pixel_size, y as f32 * pixel_size),
                        vec2(pixel_size, pixel_size),
                    );
                    painter.rect_filled(rect, 0.0, col);
                }
            }
        }

        response
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.ui_control(ui); // top controls
            ui.add_space(1.0);
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.ui_content(ui);
            });
        });
    }

    pub fn save_to_json(&self) -> String {
        serde_json::to_string(&self.pixels).unwrap()
    }

    pub fn load_from_json(&mut self, json: &str) {
        if let Ok(loaded) = serde_json::from_str::<Vec<Vec<Option<Color32>>>>(json) {
            self.pixels = loaded;
        }
    }
}
