use eframe::egui;
use crate::types::{Point, PathSegment, ResolvedShape};

pub struct ShapeViewer {
    shapes: Vec<(ResolvedShape, egui::Color32, String)>, // shape, color, name
    scale: f32,
    offset: egui::Vec2,
    show_grid: bool,
}

impl Default for ShapeViewer {
    fn default() -> Self {
        Self {
            shapes: Vec::new(),
            scale: 10.0,
            offset: egui::Vec2::new(0.0, 0.0),
            show_grid: true,
        }
    }
}

impl ShapeViewer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_shape(&mut self, shape: ResolvedShape, color: egui::Color32, name: String) {
        self.shapes.push((shape, color, name));
    }

    pub fn clear_shapes(&mut self) {
        self.shapes.clear();
    }

    fn world_to_screen(&self, point: Point, rect: egui::Rect) -> egui::Pos2 {
        let center = rect.center();
        egui::pos2(
            center.x + (point.x as f32 * self.scale) + self.offset.x,
            center.y - (point.y as f32 * self.scale) + self.offset.y, // Flip Y axis
        )
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: egui::Rect) {
        let grid_color = egui::Color32::from_gray(200);
        let axis_color = egui::Color32::from_gray(150);
        let step = self.scale;

        let center = rect.center();

        // Draw vertical lines
        let mut x = center.x % step;
        while x < rect.max.x {
            let color = if (x - center.x).abs() < 0.1 { axis_color } else { grid_color };
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                egui::Stroke::new(if color == axis_color { 2.0 } else { 1.0 }, color),
            );
            x += step;
        }

        // Draw horizontal lines
        let mut y = center.y % step;
        while y < rect.max.y {
            let color = if (y - center.y).abs() < 0.1 { axis_color } else { grid_color };
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                egui::Stroke::new(if color == axis_color { 2.0 } else { 1.0 }, color),
            );
            y += step;
        }
    }

    fn draw_path_segment(
        &self,
        painter: &egui::Painter,
        rect: egui::Rect,
        segment: &PathSegment,
        current_point: &mut Point,
        color: egui::Color32,
    ) {
        match segment {
            PathSegment::Line(start, end) => {
                let p1 = self.world_to_screen(*start, rect);
                let p2 = self.world_to_screen(*end, rect);
                painter.line_segment([p1, p2], egui::Stroke::new(2.0, color));
                *current_point = *end;
            }
            PathSegment::Arc(center, radius, start_angle, end_angle) => {
                // Draw arc using line segments
                let steps = ((end_angle - start_angle).abs() / 5.0).max(10.0) as usize;
                let angle_step = (end_angle - start_angle) / steps as f64;

                for i in 0..steps {
                    let a1 = start_angle + angle_step * i as f64;
                    let a2 = start_angle + angle_step * (i + 1) as f64;

                    let p1 = Point {
                        x: center.x + radius * a1.to_radians().cos(),
                        y: center.y + radius * a1.to_radians().sin(),
                    };
                    let p2 = Point {
                        x: center.x + radius * a2.to_radians().cos(),
                        y: center.y + radius * a2.to_radians().sin(),
                    };

                    let screen_p1 = self.world_to_screen(p1, rect);
                    let screen_p2 = self.world_to_screen(p2, rect);
                    painter.line_segment([screen_p1, screen_p2], egui::Stroke::new(2.0, color));
                }

                *current_point = Point {
                    x: center.x + radius * end_angle.to_radians().cos(),
                    y: center.y + radius * end_angle.to_radians().sin(),
                };
            }
            PathSegment::ConnectedArc(center, radius, start_angle, end_angle, _start_pt, end_pt) => {
                // Similar to Arc
                let steps = ((end_angle - start_angle).abs() / 5.0).max(10.0) as usize;
                let angle_step = (end_angle - start_angle) / steps as f64;

                for i in 0..steps {
                    let a1 = start_angle + angle_step * i as f64;
                    let a2 = start_angle + angle_step * (i + 1) as f64;

                    let p1 = Point {
                        x: center.x + radius * a1.to_radians().cos(),
                        y: center.y + radius * a1.to_radians().sin(),
                    };
                    let p2 = Point {
                        x: center.x + radius * a2.to_radians().cos(),
                        y: center.y + radius * a2.to_radians().sin(),
                    };

                    let screen_p1 = self.world_to_screen(p1, rect);
                    let screen_p2 = self.world_to_screen(p2, rect);
                    painter.line_segment([screen_p1, screen_p2], egui::Stroke::new(2.0, color));
                }

                *current_point = *end_pt;
            }
            PathSegment::ClosePath => {
                // Close path is handled automatically by tracking first point
            }
            PathSegment::DrawPoint(point) => {
                // Draw a point as a small circle
                let screen_pos = self.world_to_screen(*point, rect);
                painter.circle_filled(screen_pos, 5.0, color);
                painter.circle_stroke(screen_pos, 5.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
            }
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(ui.available_width(), ui.available_height()),
            egui::Sense::drag(),
        );

        let rect = response.rect;

        // Draw grid
        if self.show_grid {
            self.draw_grid(&painter, rect);
        }

        // Handle dragging
        if response.dragged() {
            self.offset += response.drag_delta();
        }

        // Draw all shapes
        for (shape, color, _name) in &self.shapes {
            let first_point = crate::geometry::get_starting_point(&shape.segments);
            let mut current_point = first_point.unwrap_or(Point { x: 0.0, y: 0.0 });

            for segment in &shape.segments {
                self.draw_path_segment(&painter, rect, segment, &mut current_point, *color);
            }
        }
    }
}

impl eframe::App for ShapeViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Vepor Shape Viewer");

                ui.separator();

                if ui.button("Reset View").clicked() {
                    self.offset = egui::Vec2::new(0.0, 0.0);
                    self.scale = 10.0;
                }

                ui.checkbox(&mut self.show_grid, "Show Grid");

                ui.label("Zoom:");
                ui.add(egui::Slider::new(&mut self.scale, 1.0..=50.0).text("scale"));
            });

            ui.separator();

            // Shape list
            ui.horizontal(|ui| {
                ui.label("Shapes:");
                for (_shape, color, name) in &self.shapes {
                    ui.colored_label(*color, name);
                    ui.separator();
                }
            });

            ui.separator();

            // Drawing area
            self.draw(ui);
        });
    }
}

pub fn run_viewer(shapes: Vec<(ResolvedShape, egui::Color32, String)>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Vepor - 2D Shape Viewer"),
        ..Default::default()
    };

    let mut viewer = ShapeViewer::new();
    for (shape, color, name) in shapes {
        viewer.add_shape(shape, color, name);
    }

    eframe::run_native(
        "Vepor Shape Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(viewer))),
    )
}

