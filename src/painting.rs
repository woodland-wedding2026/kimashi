use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, emath, NumExt};
use serde::{Serialize, Deserialize};

/// Serializable version of Stroke (egui::Stroke is not serializable)
#[derive(Serialize, Deserialize)]
pub struct SerializableStroke {
    pub width: f32,
    pub color: [u8; 4], // RGBA
}

#[derive(Serialize)]
pub struct ExportedPainting {
    pub theme: String, // "light" or "dark"
    pub lines: Vec<SerializableLine>,
}



impl From<Stroke> for SerializableStroke {
    fn from(stroke: Stroke) -> Self {
        Self {
            width: stroke.width,
            color: stroke.color.to_array(),
        }
    }
}

impl From<SerializableStroke> for Stroke {
    fn from(s: SerializableStroke) -> Self {
        Stroke {
            width: s.width,
            color: Color32::from_rgba_unmultiplied(s.color[0], s.color[1], s.color[2], s.color[3]),
        }
    }
}

/// Serializable version of a painted line
#[derive(Serialize, Deserialize)]
pub struct SerializableLine {
    pub points: Vec<[f32; 2]>,
    pub stroke: SerializableStroke,
}

/// Main Painting App
#[derive(Default)]
pub struct PaintingApp {
    pub lines: Vec<(Vec<Pos2>, Stroke)>,
    pub stroke: Stroke,
}

impl PaintingApp {
    /// UI controls: stroke width, clear, save

    pub fn export_json(&self, ctx: &egui::Context) -> Option<String> {
        let serializable_lines: Vec<SerializableLine> = self.lines.iter().map(|(points, stroke)| {
            SerializableLine {
                points: points.iter().map(|p| [p.x, p.y]).collect(),
                stroke: (*stroke).into(),
            }
        }).collect();
    
        let theme = if ctx.style().visuals.dark_mode {
            "dark".to_string()
        } else {
            "light".to_string()
        };
    
        let export = ExportedPainting {
            theme,
            lines: serializable_lines,
        };
    
        serde_json::to_string_pretty(&export).ok()
    }


    
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label("Stroke:");
            ui.add(&mut self.stroke);
            ui.separator();

            if ui.button("Clear Painting").clicked() {
                self.lines.clear();
            }

            if ui.button("Save Painting").clicked() {
                self.save_to_json();
            }
        }).response
    }

    /// Paint canvas interaction and drawing
    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (mut response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        // Set default stroke on first frame
        if self.stroke.width == 0.0 {
            self.stroke = Stroke {
                width: 1.0,
                color: Color32::from_rgb(255, 255, 0),
                ..Default::default()
            };
        }

        if self.lines.is_empty() {
            self.lines.push((vec![], self.stroke));
        }

        let (current_line, _) = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push((vec![], self.stroke));
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

    /// Top-level egui UI entry
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.ui_control(ui);
            ui.add_space(1.0);
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.ui_content(ui);
            });
        });
    }

    /// Save current painting as JSON (printed to console)
    fn save_to_json(&self) {
        let serializable_lines: Vec<SerializableLine> = self.lines.iter().map(|(points, stroke)| {
            SerializableLine {
                points: points.iter().map(|p| [p.x, p.y]).collect(),
                stroke: (*stroke).into(),
            }
        }).collect();

        match serde_json::to_string_pretty(&serializable_lines) {
            Ok(json) => {
                println!("Painting JSON:\n{}", json);
                // Optional: write to file or clipboard
            }
            Err(err) => {
                eprintln!("❌ Failed to serialize painting: {}", err);
            }
        }
    }
}
