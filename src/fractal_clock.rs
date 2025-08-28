use egui::{Painter, Pos2, Rect, Stroke, Color32, StrokeKind};
use std::f32::consts::TAU;

pub struct FractalClock {
    pub depth: usize,
    pub thickness: f32,
    pub length_factor: f32,
    pub time_scale: f64,
}


impl Default for FractalClock {
    fn default() -> Self {
        Self {
            depth: 4,
            thickness: 1.5,
            length_factor: 0.65,
            time_scale: 1.0,
        }
    }
}

impl FractalClock {
    pub fn paint(&self, painter: &Painter, rect: Rect, time_seconds: f64) {
        let time = time_seconds * self.time_scale;
        draw_fractal_clock(
            painter,
            rect,
            time,
            self.depth,
            self.thickness,
            self.length_factor,
        );
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Fractal Clock Settings", |ui| {
            ui.add(egui::Slider::new(&mut self.depth, 0..=10).text("Recursion Depth"));
            ui.add(egui::Slider::new(&mut self.thickness, 0.1..=5.0).text("Line Thickness"));
            ui.add(egui::Slider::new(&mut self.length_factor, 0.1..=0.9).text("Length Factor"));
            ui.add(egui::Slider::new(&mut self.time_scale, 0.1..=10.0).text("Time Scale"));
        });
    }
}

fn total_branch_length(depth: usize, base_length: f32, length_factor: f32) -> f32 {
    let mut total = 0.0;
    let mut length = base_length;
    for _ in 0..=depth {
        total += length;
        length *= length_factor;
    }
    total
}




fn draw_fractal_clock(
    painter: &Painter,
    rect: Rect,
    seconds: f64,
    depth: usize,
    thickness: f32,
    length_factor: f32,
) {
    let center = rect.center();
    let max_allowed_radius = 0.5 * rect.size().min_elem();
    let estimated_base = 100.0;
    let total_len = total_branch_length(depth, estimated_base, length_factor);
    let scaling_factor = max_allowed_radius / total_len;
    let radius = estimated_base * scaling_factor;

    painter.rect_stroke(rect, 0.0, Stroke::new(1.0, Color32::DARK_GRAY), StrokeKind::Inside);

    draw_branch(
        painter,
        center,
        radius,
        TAU * seconds as f32 / 60.0,
        0,
        depth,
        thickness,
        length_factor,
    );
}


fn draw_branch(
    painter: &Painter,
    origin: Pos2,
    length: f32,
    angle: f32,
    current_depth: usize,
    max_depth: usize,
    thickness: f32,
    length_factor: f32,
) {
    if current_depth > max_depth || length < 2.0 {
        return;
    }

    let dir = egui::vec2(angle.cos(), angle.sin());
    let tip = origin + dir * length;

    let adjusted_thickness = thickness - (current_depth as f32 * 0.1);
    painter.line_segment(
        [origin, tip],
        Stroke::new(adjusted_thickness.max(0.5), Color32::WHITE),
    );

    let new_length = length * length_factor;
    let child_angle_offset = TAU / 6.0;

    draw_branch(
        painter,
        tip,
        new_length,
        angle + child_angle_offset,
        current_depth + 1,
        max_depth,
        thickness,
        length_factor,
    );
    draw_branch(
        painter,
        tip,
        new_length,
        angle - child_angle_offset,
        current_depth + 1,
        max_depth,
        thickness,
        length_factor,
    );
}
