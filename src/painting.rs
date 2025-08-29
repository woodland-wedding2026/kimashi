use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, emath, NumExt};

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct PaintingApp {
    pub lines: Vec<Vec<Pos2>>,
    pub stroke: Stroke,
}

impl PaintingApp {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label("Stroke:");
            ui.add(&mut self.stroke);
            ui.separator();
            if ui.button("Clear Painting").clicked() {
                self.lines.clear();
                self.lines.push(vec![]);
            }
        })
        .response
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let desired_size = ui.available_size_before_wrap().at_least(egui::vec2(300.0, 300.0));
        let (mut response, painter) = ui.allocate_painter(desired_size, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        if response.dragged() {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                let canvas_pos = from_screen * pointer_pos;
                if current_line.last() != Some(&canvas_pos) {
                    current_line.push(canvas_pos);
                    response.mark_changed();
                }
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
            response.mark_changed();
        }

        let shapes = self
            .lines
            .iter()
            .filter(|line| line.len() >= 2)
            .map(|line| {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                egui::Shape::line(points, self.stroke)
            });

        painter.extend(shapes);

        response
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
        self.ui_control(ui); // top bar with stroke + clear

        // Add some spacing
        ui.add_space(1.0);

        // Fill remaining space for the painting canvas
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            self.ui_content(ui);
        });
    });
    }
}
