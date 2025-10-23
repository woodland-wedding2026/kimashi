use egui::{Painter, Pos2, Rect, Stroke, Color32, StrokeKind};
use std::f32::consts::TAU;

/// Main struct to control fractal clock parameters.
pub struct FractalClock {
    pub depth: usize,
    pub thickness: f32,
    pub length_factor: f32,
    pub time_scale: f64,
    //pub language_flag: bool,
}

impl Default for FractalClock {
    fn default() -> Self {
        Self {
            depth: 5,
            thickness: 1.5,
            length_factor: 0.65,
            time_scale: 1.0,
            //language_flag: true,
            
        }
    }
}

impl FractalClock {
    /// Call this from your `App::update()` to draw the fractal clock.
    pub fn paint(&self, painter: &Painter, rect: Rect, time_seconds: f64) {
        let scaled_time = time_seconds * self.time_scale;
        draw_fractal_clock(
            painter,
            rect,
            scaled_time,
            self.depth,
            self.thickness,
            self.length_factor,
        );
    }

    /// UI controls for the fractal clock.
    pub fn ui(&mut self, ui: &mut egui::Ui, language_flag: bool) {
        if language_flag == true {
                        ui.collapsing("settings", |ui| {
            ui.add(egui::Slider::new(&mut self.depth, 0..=10).text("Recursion Depth"));
            ui.add(egui::Slider::new(&mut self.thickness, 0.1..=5.0).text("Line Thickness"));
            ui.add(egui::Slider::new(&mut self.length_factor, 0.1..=0.9).text("Length Factor"));
            ui.add(egui::Slider::new(&mut self.time_scale, 0.1..=10.0).text("Time Scale"));
        });
                    }
                    else {
                        ui.collapsing("Einstellungen", |ui| {
            ui.add(egui::Slider::new(&mut self.depth, 0..=10).text("Rekursionstiefe"));
            ui.add(egui::Slider::new(&mut self.thickness, 0.1..=5.0).text("Liniendicke"));
            ui.add(egui::Slider::new(&mut self.length_factor, 0.1..=0.9).text("Längenfaktor"));
            ui.add(egui::Slider::new(&mut self.time_scale, 0.1..=10.0).text("Zeitskala"));
        });
                    }
        //ui.collapsing("settings", |ui| {
        //    ui.add(egui::Slider::new(&mut self.depth, 0..=10).text("Recursion Depth"));
        //    ui.add(egui::Slider::new(&mut self.thickness, 0.1..=5.0).text("Line Thickness"));
        //    ui.add(egui::Slider::new(&mut self.length_factor, 0.1..=0.9).text("Length Factor"));
        //    ui.add(egui::Slider::new(&mut self.time_scale, 0.1..=10.0).text("Time Scale"));
        //});
    }
}

/// Compute total length of all branches to scale the base length.
fn total_branch_length(depth: usize, base_length: f32, length_factor: f32) -> f32 {
    let mut total = 0.0;
    let mut length = base_length;
    for _ in 0..=depth {
        total += length;
        length *= length_factor;
    }
    total
}

/// Main drawing function for the fractal clock.
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

    // Optional: draw bounding box
    painter.rect_stroke(rect, 0.0, Stroke::new(1.0, Color32::DARK_GRAY), StrokeKind::Inside);

    let time = egui_time_components(seconds);

    // Draw recursive fractal branches for hour, minute, and second hands
    draw_branch_recursive(
        painter,
        center,
        radius,
        time.hours_angle,
        0,
        depth,
        thickness,
        length_factor,
        Color32::from_rgb(200, 200, 255), // Blueish
    );
    draw_branch_recursive(
        painter,
        center,
        radius,
        time.minutes_angle,
        0,
        depth,
        thickness,
        length_factor,
        Color32::from_rgb(200, 255, 200), // Greenish
    );
    draw_branch_recursive(
        painter,
        center,
        radius,
        time.seconds_angle,
        0,
        depth,
        thickness,
        length_factor,
        Color32::from_rgb(255, 200, 200), // Reddish
    );
}

/// Time angles (in radians) for hour, minute, and second hands.
struct TimeAngles {
    hours_angle: f32,
    minutes_angle: f32,
    seconds_angle: f32,
}

/// Converts raw seconds into individual hand angles.
fn egui_time_components(seconds: f64) -> TimeAngles {
    let sec = seconds % 60.0;
    let min = (seconds / 60.0) % 60.0;
    let hour = (seconds / 3600.0) % 12.0;

    TimeAngles {
        seconds_angle: (sec as f32 / 60.0) * TAU,
        minutes_angle: (min as f32 / 60.0) * TAU,
        hours_angle: (hour as f32 / 12.0) * TAU,
    }
}

/// Recursive branch drawing with color fade and dynamic angles.
fn draw_branch_recursive(
    painter: &Painter,
    origin: Pos2,
    length: f32,
    angle: f32,
    current_depth: usize,
    max_depth: usize,
    base_thickness: f32,
    length_factor: f32,
    base_color: Color32,
) {
    if current_depth > max_depth || length < 1.0 {
        return;
    }

    let dir = egui::vec2(angle.cos(), angle.sin());
    let tip = origin + dir * length;

    let thickness = base_thickness * (1.0 - current_depth as f32 * 0.1).max(0.3);
    let fade = (255.0 * (1.0 - current_depth as f32 / (max_depth as f32 + 1.0))) as u8;
    let color = Color32::from_rgba_premultiplied(
        base_color.r(),
        base_color.g(),
        base_color.b(),
        fade,
    );

    painter.line_segment([origin, tip], Stroke::new(thickness, color));

    let next_length = length * length_factor;

    // Use depth-dependent angle spread
    let spread = angle_delta(current_depth);

    draw_branch_recursive(
        painter,
        tip,
        next_length,
        angle + spread,
        current_depth + 1,
        max_depth,
        base_thickness,
        length_factor,
        base_color,
    );
    draw_branch_recursive(
        painter,
        tip,
        next_length,
        angle - spread,
        current_depth + 1,
        max_depth,
        base_thickness,
        length_factor,
        base_color,
    );
}

/// Controls how widely the branches fan out as recursion deepens.
fn angle_delta(depth: usize) -> f32 {
    // Start wide, narrow with depth
    TAU / 12.0 * (1.0 - (depth as f32 * 0.07)).max(0.1)
}
