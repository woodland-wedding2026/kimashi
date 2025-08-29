use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, emath};

pub struct PaintingApp {
    lines: Vec<Vec<Pos2>>,
    stroke: Stroke,
}

impl Default for PaintingApp {
    fn default() -> Self {
        Self {
            lines: vec![vec![]],
            stroke: Stroke::new(2.0, Color32::WHITE),
        }
    }
}

impl PaintingApp {
    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        let current_line = self.lines.last_mut().unwrap();

        if let Some(pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pos;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
            response.mark_changed();
        }

        for line in &self.lines {
            if line.len() >= 2 {
                let points: Vec<_> = line.iter().map(|p| to_screen * *p).collect();
                painter.line(points, self.stroke);
            }
        }

        response
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Stroke width:");
                ui.add(egui::Slider::new(&mut self.stroke.width, 1.0..=10.0));
                if ui.button("Clear").clicked() {
                    self.lines.clear();
                    self.lines.push(vec![]);
                }
            });
            ui.add_space(8.0);
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.ui_content(ui);
            });
        });
    }
}
