use egui::{Painter, Pos2, Rect, Stroke, Color32, StrokeKind};
use std::f32::consts::TAU;
use std::time::Instant;


pub struct FractalClock {
    pub start_time: Instant,
}

impl Default for FractalClock {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }
}

impl FractalClock {
    pub fn paint(&self, painter: &Painter, rect: Rect) {
        let now = Instant::now();
        let seconds = (now - self.start_time).as_secs_f64();

        draw_fractal_clock(painter, rect, seconds);
    }
}

fn draw_fractal_clock(painter: &Painter, rect: Rect, seconds: f64) {
    let center = rect.center();
    let radius = 0.4 * rect.size().min_elem();
    let time = seconds;

    painter.rect_stroke(rect, 0.0, Stroke::new(1.0, Color32::DARK_GRAY), StrokeKind::Inside );

    // Start the recursion
    draw_branch(
        painter,
        center,
        radius,
        TAU * time as f32 / 60.0,
        0,
    );
}

fn draw_branch(
    painter: &Painter,
    origin: Pos2,
    length: f32,
    angle: f32,
    depth: usize,
) {
    if depth > 10 || length < 2.0 {
        return;
    }

    let dir = egui::vec2(angle.cos(), angle.sin());
    let tip = origin + dir * length;

    let thickness = 1.5 - (depth as f32 * 0.1);
    painter.line_segment(
        [origin, tip],
        Stroke::new(thickness.max(0.5), Color32::WHITE),
    );

    let new_length = length * 0.65;
    let child_angle_offset = TAU / 6.0;

    draw_branch(painter, tip, new_length, angle + child_angle_offset, depth + 1);
    draw_branch(painter, tip, new_length, angle - child_angle_offset, depth + 1);
}
