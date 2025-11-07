use crate::types::{PathSegment, Point, ResolvedShape};
use eframe::egui;
use std::sync::Arc;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tool {
    Hand, // For panning and selecting
    Circle,
    Rectangle,
    Intersection,
    Union,
    Difference,
    Xor,
    MeasureArea,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum DrawingState {
    None,
    CircleFirstClick(Point),
    RectangleFirstClick(Point),
}

pub struct ShapeViewer {
    shapes: Vec<(ResolvedShape, egui::Color32, String)>, // shape, color, name
    scale: f32,
    offset: egui::Vec2,
    show_grid: bool,
    show_control_points: bool,
    snap_threshold: f32,
    previous_scale: f32, // 用于跟踪 scale 的变化
    selected_tool: Tool,
    drawing_state: DrawingState,
    was_dragged: bool,
}

impl Default for ShapeViewer {
    fn default() -> Self {
        Self {
            shapes: Vec::new(),
            scale: 10.0,
            offset: egui::Vec2::new(0.0, 0.0),
            show_grid: true,
            show_control_points: true,
            snap_threshold: 10.0,
            previous_scale: 10.0,
            selected_tool: Tool::Hand,
            drawing_state: DrawingState::None,
            was_dragged: false,
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

    fn screen_to_world(&self, pos: egui::Pos2, rect: egui::Rect) -> Point {
        let center = rect.center();
        Point {
            x: ((pos.x - center.x - self.offset.x) / self.scale) as f64,
            y: ((center.y - pos.y + self.offset.y) / self.scale) as f64, // Flip Y axis
        }
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: egui::Rect) {
        let grid_color = egui::Color32::from_gray(if painter.ctx().style().visuals.dark_mode {
            50
        } else {
            200
        });
        let axis_color = egui::Color32::from_gray(150);
        let step = self.scale;

        // When zoomed out too far, drawing a grid is messy and slow.
        if step < 4.0 {
            return;
        }

        let screen_origin = rect.center() + self.offset;

        // Draw vertical lines
        if step > 0.0 {
            let start_k = ((rect.min.x - screen_origin.x) / step).floor() as i32;
            let end_k = ((rect.max.x - screen_origin.x) / step).ceil() as i32;

            for k in start_k..=end_k {
                let x = screen_origin.x + k as f32 * step;
                let is_axis = k == 0;
                let color = if is_axis { axis_color } else { grid_color };
                painter.line_segment(
                    [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                    egui::Stroke::new(if is_axis { 2.0 } else { 1.0 }, color),
                );
            }
        }

        // Draw horizontal lines
        if step > 0.0 {
            // world_y is inverted on screen: screen_y = screen_origin.y - world_y * step
            // So, world_y = (screen_origin.y - screen_y) / step
            // We need to find the integer range for world_y (k) that is visible on screen.
            let start_k = ((screen_origin.y - rect.max.y) / step).floor() as i32;
            let end_k = ((screen_origin.y - rect.min.y) / step).ceil() as i32;

            for k in start_k..=end_k {
                let y = screen_origin.y - k as f32 * step;
                let is_axis = k == 0;
                let color = if is_axis { axis_color } else { grid_color };
                painter.line_segment(
                    [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                    egui::Stroke::new(if is_axis { 2.0 } else { 1.0 }, color),
                );
            }
        }
    }

    fn get_all_control_points(&self) -> Vec<Point> {
        let mut all_control_points = Vec::new();
        for (shape, _, _) in &self.shapes {
            all_control_points.extend(self.get_control_points(shape));
        }
        all_control_points
    }

    fn get_control_points(&self, shape: &ResolvedShape) -> Vec<Point> {
        let mut control_points = Vec::new();
        for segment in &shape.segments {
            match segment {
                PathSegment::Line(start, end) => {
                    control_points.push(*start);
                    control_points.push(*end);
                }
                PathSegment::Arc(center, _, _, _) => {
                    control_points.push(*center);
                }
                PathSegment::ConnectedArc(center, _, _, _, start_pt, end_pt) => {
                    control_points.push(*center);
                    control_points.push(*start_pt);
                    control_points.push(*end_pt);
                }
                _ => {}
            }
        }
        control_points
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
                let steps = ((end_angle - start_angle).abs() / 5.0).max(30.0) as usize;
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
            PathSegment::ConnectedArc(
                center,
                radius,
                start_angle,
                end_angle,
                _start_pt,
                end_pt,
            ) => {
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
                painter.circle_stroke(
                    screen_pos,
                    5.0,
                    egui::Stroke::new(1.0, egui::Color32::BLACK),
                );
            }
        }
    }

    fn create_shape(&mut self, end_point: Point) {
        match self.drawing_state {
            DrawingState::CircleFirstClick(center) => {
                let radius = center.distance_to(end_point);
                let new_shape = ResolvedShape {
                    segments: vec![PathSegment::Arc(center, radius, 0.0, 360.0)],
                };
                self.add_shape(
                    new_shape,
                    egui::Color32::from_rgb(255, 0, 0),
                    "Circle".to_string(),
                );
            }
            DrawingState::RectangleFirstClick(p1) => {
                let p2 = end_point;
                let top_left = Point {
                    x: p1.x.min(p2.x),
                    y: p1.y.max(p2.y),
                };
                let bottom_right = Point {
                    x: p1.x.max(p2.x),
                    y: p1.y.min(p2.y),
                };
                let top_right = Point {
                    x: bottom_right.x,
                    y: top_left.y,
                };
                let bottom_left = Point {
                    x: top_left.x,
                    y: bottom_right.y,
                };

                let new_shape = ResolvedShape {
                    segments: vec![
                        PathSegment::Line(top_left, top_right),
                        PathSegment::Line(top_right, bottom_right),
                        PathSegment::Line(bottom_right, bottom_left),
                        PathSegment::Line(bottom_left, top_left),
                    ],
                };
                self.add_shape(
                    new_shape,
                    egui::Color32::from_rgb(0, 255, 0),
                    "Rectangle".to_string(),
                );
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(ui.available_width(), ui.available_height()),
            egui::Sense::click_and_drag(),
        );

        let rect = response.rect;

        // Draw grid
        if self.show_grid {
            self.draw_grid(&painter, rect);
        }

        // Handle input for panning and drawing
        if response.drag_started() {
            if self.selected_tool == Tool::Hand {
                // Pan
            } else if self.selected_tool == Tool::Circle || self.selected_tool == Tool::Rectangle {
                if let Some(mouse_pos) = response.hover_pos() {
                    let world_pos = self.screen_to_world(mouse_pos, rect);
                    self.drawing_state = match self.selected_tool {
                        Tool::Circle => DrawingState::CircleFirstClick(world_pos),
                        Tool::Rectangle => DrawingState::RectangleFirstClick(world_pos),
                        _ => DrawingState::None,
                    };
                }
            }
        } else if response.drag_stopped() {
            if self.was_dragged {
                if let Some(mouse_pos) = response.hover_pos() {
                    let world_pos = self.screen_to_world(mouse_pos, rect);
                    self.create_shape(world_pos);
                    self.drawing_state = DrawingState::None;
                }
                self.was_dragged = false;
            }
        } else if response.clicked() {
            if let Some(mouse_pos) = response.hover_pos() {
                let mut world_pos = self.screen_to_world(mouse_pos, rect);

                // Snap to control points
                if self.show_control_points {
                    let all_control_points = self.get_all_control_points();
                    for point in all_control_points {
                        let screen_point = self.world_to_screen(point, rect);
                        if mouse_pos.distance(screen_point) < self.snap_threshold {
                            world_pos = point;
                            break;
                        }
                    }
                }

                match self.drawing_state {
                    DrawingState::None => {
                        self.drawing_state = match self.selected_tool {
                            Tool::Circle => DrawingState::CircleFirstClick(world_pos),
                            Tool::Rectangle => DrawingState::RectangleFirstClick(world_pos),
                            _ => DrawingState::None,
                        };
                    }
                    _ => {
                        self.create_shape(world_pos);
                        self.drawing_state = DrawingState::None;
                    }
                }
            }
        }

        if response.dragged() {
            if self.selected_tool == Tool::Hand {
                self.offset += response.drag_delta();
            }
            else if self.selected_tool == Tool::Circle || self.selected_tool == Tool::Rectangle {
                self.was_dragged = true;
            }
        }

        // Draw preview of shape being drawn
        if let Some(mouse_pos) = response.hover_pos() {
            let mut world_pos = self.screen_to_world(mouse_pos, rect);

            // Snap to control points
            if self.show_control_points {
                let all_control_points = self.get_all_control_points();
                for point in all_control_points {
                    let screen_point = self.world_to_screen(point, rect);
                    if mouse_pos.distance(screen_point) < self.snap_threshold {
                        world_pos = point;
                        break;
                    }
                }
            }

            match self.drawing_state {
                DrawingState::CircleFirstClick(center) => {
                    let radius = center.distance_to(world_pos);
                    let circle = egui::Shape::circle_stroke(
                        self.world_to_screen(center, rect),
                        radius as f32 * self.scale,
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 0, 0)),
                    );
                    painter.add(circle);
                }
                DrawingState::RectangleFirstClick(p1) => {
                    let p2 = world_pos;
                    let screen_p1 = self.world_to_screen(p1, rect);
                    let screen_p2 = self.world_to_screen(p2, rect);
                    painter.rect_stroke(
                        egui::Rect::from_points(&[screen_p1, screen_p2]),
                        egui::CornerRadius::ZERO,
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 0)),
                        egui::StrokeKind::Middle,
                    );
                }
                _ => {}
            }
        }

        // Draw all shapes
        for (shape, color, _name) in &self.shapes {
            let first_point = crate::geometry::get_starting_point(&shape.segments);
            let mut current_point = first_point.unwrap_or(Point { x: 0.0, y: 0.0 });

            for segment in &shape.segments {
                self.draw_path_segment(&painter, rect, segment, &mut current_point, *color);
            }
        }

        // Draw control points if enabled
        if self.show_control_points {
            for (shape, color, _name) in &self.shapes {
                let control_points = self.get_control_points(shape);
                for point in control_points {
                    self.draw_path_segment(
                        &painter,
                        rect,
                        &PathSegment::DrawPoint(point),
                        &mut Point { x: 0.0, y: 0.0 }, // current_point is not used for DrawPoint
                        *color,
                    );
                }
            }
        }
    }
}

impl eframe::App for ShapeViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 处理触摸板手势：
        // - 双指滚动 (scroll_delta) 视为平移
        // - 双指捏合 (zoom_delta) 视为缩放（保持视觉中心）
        // egui::Context::input 在较新版本中需要一个闭包来读取 InputState
        // 我们只需要 scroll_delta 和 zoom_delta，因此在闭包中读取并复制出来
        let (scroll_delta, zoom) =
            ctx.input(|input_state| (input_state.smooth_scroll_delta, input_state.zoom_delta()));

        // 平移（双指滑动）
        if scroll_delta != egui::Vec2::ZERO {
            // 直接把滚动偏移应用到视图偏移上
            self.offset += scroll_delta;
        }

        // 缩放（捏合手势）
        // egui 的 zoom_delta 通常以 1.0 为无变化（或接近），当有捏合时会返回大于或小于 1.0 的值
        let zoom = zoom;
        if (zoom - 1.0).abs() > std::f32::EPSILON {
            let old_scale = self.scale;
            self.scale *= zoom;
            // 同步 offset 以保持视觉中心（与 slider 的行为一致）
            let scale_ratio = if old_scale.abs() > std::f32::EPSILON {
                self.scale / old_scale
            } else {
                1.0
            };
            self.offset = self.offset * scale_ratio;
            self.previous_scale = self.scale;
        }

        // Handle escape key to cancel drawing
        ctx.input(|input_state| {
            if input_state.key_pressed(egui::Key::Escape) {
                self.drawing_state = DrawingState::None;
            }
        });

        egui::SidePanel::left("toolbar")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("工具");
                ui.separator();

                let previous_tool = self.selected_tool;

                ui.selectable_value(&mut self.selected_tool, Tool::Hand, "移动画布");
                ui.selectable_value(&mut self.selected_tool, Tool::Circle, "画圆");
                ui.selectable_value(&mut self.selected_tool, Tool::Rectangle, "画矩形");
                ui.selectable_value(&mut self.selected_tool, Tool::Intersection, "取交点");
                ui.selectable_value(&mut self.selected_tool, Tool::Union, "取并集");
                ui.selectable_value(&mut self.selected_tool, Tool::Difference, "取差集");
                ui.selectable_value(&mut self.selected_tool, Tool::Xor, "取异或");
                ui.selectable_value(&mut self.selected_tool, Tool::MeasureArea, "测量面积");

                if self.selected_tool != previous_tool {
                    self.drawing_state = DrawingState::None;
                }

                ui.separator();

                // 显示当前选择的工具
                ui.label(format!("当前工具: {:?}", self.selected_tool));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Vepor Shape Viewer");

                ui.separator();

                if ui.button("Reset View").clicked() {
                    self.offset = egui::Vec2::new(0.0, 0.0);
                    self.scale = 10.0;
                    self.previous_scale = 10.0;
                }

                ui.checkbox(&mut self.show_grid, "Show Grid");
                ui.checkbox(&mut self.show_control_points, "Show Control Points");

                ui.label("Zoom:");
                let old_scale = self.scale;
                ui.add(egui::Slider::new(&mut self.scale, 1.0..=50.0).text("scale"));

                // 当 scale 改变时，调整 offset 以保持视觉中心不变
                if (self.scale - old_scale).abs() > 0.001 {
                    let scale_ratio = self.scale / old_scale;
                    self.offset = self.offset * scale_ratio;
                    self.previous_scale = self.scale;
                }
            });

            ui.separator();

            // Shape list
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Shapes:");
                    for (_shape, color, name) in &self.shapes {
                        ui.colored_label(*color, name);
                        ui.separator();
                    }
                });
            });

            ui.separator();

            // Drawing area
            self.draw(ui);
        });
    }
}

pub fn run_viewer(
    shapes: Vec<(ResolvedShape, egui::Color32, String)>,
) -> Result<(), eframe::Error> {
    // 创建图标
    let icon = crate::icon::get_icon_data();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Vepor - 2D Shape Viewer")
            .with_icon(Arc::new(icon)),
        ..Default::default()
    };

    let mut viewer = ShapeViewer::new();
    for (shape, color, name) in shapes {
        viewer.add_shape(shape, color, name);
    }

    eframe::run_native(
        "Vepor Shape Viewer",
        options,
        Box::new(|cc| {
            setup_chinese_fonts(&cc.egui_ctx);
            Ok(Box::new(viewer))
        }),
    )
}

/// 设置中文字体支持
fn setup_chinese_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    println!("开始加载中文字体...");

    // 尝试多个可能的中文字体路径
    let font_paths = vec![
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/Supplemental/Songti.ttc",
        "/System/Library/Fonts/Supplemental/STHeiti Light.ttc",
        "/System/Library/Fonts/Supplemental/STHeiti Medium.ttc",
        "/Library/Fonts/Arial Unicode.ttf",
    ];

    let mut font_loaded = false;
    for (idx, font_path) in font_paths.iter().enumerate() {
        println!("尝试加载字体: {}", font_path);
        if let Ok(font_data) = std::fs::read(font_path) {
            println!(
                "成功读取字体文件: {} ({} bytes)",
                font_path,
                font_data.len()
            );
            let font_name = format!("ChineseFont{}", idx);
            fonts.font_data.insert(
                font_name.clone(),
                Arc::new(egui::FontData::from_owned(font_data)),
            );

            // 将中文字体添加到字体家族（放在最前面以优先使用）
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, font_name.clone());

            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, font_name);

            font_loaded = true;
            println!("✓ 成功加载字体: {}", font_path);
            break;
        } else {
            println!("✗ 无法读取字体: {}", font_path);
        }
    }

    if !font_loaded {
        eprintln!("警告: 未能加载任何中文字体，中文可能无法正常显示");
        eprintln!("请确保系统中存在以下字体之一:");
        for path in &font_paths {
            eprintln!("  - {}", path);
        }
    }

    ctx.set_fonts(fonts);
    println!("字体设置完成");
}
