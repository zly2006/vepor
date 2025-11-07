mod types;
mod geometry;
mod intersection;
mod boolean_ops;
mod resolver;
mod viewer;

use crate::types::{PathSegment, ResolvedShape, Point};
use crate::intersection::arc_arc_intersection;

/// 计算两个圆的交点
fn circle_circle_intersection(center1: Point, radius1: f64, center2: Point, radius2: f64) -> Vec<Point> {
    // 使用完整的圆弧 (0到360度) 来找到交点
    arc_arc_intersection(center1, radius1, 0.0, 360.0, center2, radius2, 0.0, 360.0)
}

fn main() {
    // 定义两个圆
    let circle1_center = Point { x: 0.0, y: 0.0 };
    let circle1_radius = 5.0;
    
    let circle2_center = Point { x: 6.0, y: 0.0 };
    let circle2_radius = 4.0;

    // 计算交点
    let intersections = circle_circle_intersection(
        circle1_center,
        circle1_radius,
        circle2_center,
        circle2_radius
    );

    println!("两圆交点数量: {}", intersections.len());
    for (i, point) in intersections.iter().enumerate() {
        println!("交点 {}: ({:.4}, {:.4})", i + 1, point.x, point.y);
    }

    // 创建第一个圆的路径段
    let circle1_segments = vec![
        PathSegment::Arc(circle1_center, circle1_radius, 0.0, 360.0),
    ];

    // 创建第二个圆的路径段
    let circle2_segments = vec![
        PathSegment::Arc(circle2_center, circle2_radius, 0.0, 360.0),
    ];

    // 创建交点的路径段
    let mut intersection_segments = Vec::new();
    for point in &intersections {
        intersection_segments.push(PathSegment::DrawPoint(*point));
    }

    // 准备可视化的形状列表
    let mut shapes = vec![
        (
            ResolvedShape { segments: circle1_segments },
            egui::Color32::BLUE,
            "圆1 (中心: (0, 0), 半径: 5)".to_string()
        ),
        (
            ResolvedShape { segments: circle2_segments },
            egui::Color32::GREEN,
            "圆2 (中心: (6, 0), 半径: 4)".to_string()
        ),
    ];

    // 如果有交点，添加到可视化中
    if !intersection_segments.is_empty() {
        shapes.push((
            ResolvedShape { segments: intersection_segments },
            egui::Color32::RED,
            format!("交点 ({}个)", intersections.len())
        ));
    }

    // 运行可视化窗口
    viewer::run_viewer(shapes).unwrap();
}

