mod boolean_ops;
mod geometry;
mod icon;
mod intersection;
mod resolver;
mod types;
mod viewer;

use crate::intersection::arc_arc_intersection;
use crate::types::{PathSegment, Point, ResolvedShape};

/// 计算两个圆的交点
fn circle_circle_intersection(
    center1: Point,
    radius1: f64,
    center2: Point,
    radius2: f64,
) -> Vec<Point> {
    // 使用完整的圆弧 (0到360度) 来找到交点
    arc_arc_intersection(center1, radius1, 0.0, 360.0, center2, radius2, 0.0, 360.0)
}

fn main() {
    println!("=== 圆与圆弧交点计算程序 ===\n");

    // 定义第一个圆（完整的圆）
    let circle1_center = Point { x: 0.0, y: 0.0 };
    let circle1_radius = 5.0;

    // 定义第二个形状（圆弧，而不是完整的圆）
    let circle2_center = Point { x: 6.0, y: 0.0 };
    let circle2_radius = 4.0;
    // 圆弧范围：从0度到180度（上半圆），这样只会与圆1产生一个交点
    let arc_start = 0.0;
    let arc_end = 180.0;

    println!(
        "圆1（蓝色）: 中心 ({}, {}), 半径 {} [完整圆]",
        circle1_center.x, circle1_center.y, circle1_radius
    );
    println!(
        "圆弧（绿色）: 中心 ({}, {}), 半径 {} [范围: {}° - {}°]",
        circle2_center.x, circle2_center.y, circle2_radius, arc_start, arc_end
    );
    println!();

    // 计算圆与圆弧的交点
    let intersections = arc_arc_intersection(
        circle1_center,
        circle1_radius,
        0.0,
        360.0, // 圆1是完整的圆
        circle2_center,
        circle2_radius,
        arc_start,
        arc_end, // 圆2只显示上半圆弧
    );

    println!("交点数量: {}", intersections.len());
    for (i, point) in intersections.iter().enumerate() {
        println!("交点 {}: ({:.4}, {:.4})", i + 1, point.x, point.y);
    }
    println!();

    // 创建第一个圆的路径段（完整圆）
    let circle1_segments = vec![PathSegment::Arc(circle1_center, circle1_radius, 0.0, 360.0)];

    // 创建第二个圆的路径段（圆弧，不是完整圆）
    let arc_segments = vec![PathSegment::Arc(
        circle2_center,
        circle2_radius,
        arc_start,
        arc_end,
    )];

    // 创建交点的路径段
    let mut intersection_segments = Vec::new();
    for point in &intersections {
        intersection_segments.push(PathSegment::DrawPoint(*point));
    }

    // 准备可视化的形状列表
    let mut shapes = vec![
        (
            ResolvedShape {
                segments: circle1_segments,
            },
            egui::Color32::BLUE,
            "圆1 (完整圆)".to_string(),
        ),
        (
            ResolvedShape {
                segments: arc_segments,
            },
            egui::Color32::GREEN,
            format!("圆弧 ({}° - {}°)", arc_start, arc_end),
        ),
    ];

    // 如果有交点，添加到可视化中
    if !intersection_segments.is_empty() {
        shapes.push((
            ResolvedShape {
                segments: intersection_segments,
            },
            egui::Color32::RED,
            format!("交点 ({}个)", intersections.len()),
        ));
    }

    println!("正在启动可视化窗口...");
    println!("提示: 可以拖拽画布移动视图，使用Zoom滑块缩放");
    println!("      绿色圆弧只显示上半部分，因此只有一个交点\n");

    // 运行可视化窗口
    if let Err(e) = viewer::run_viewer(shapes) {
        eprintln!("运行可视化窗口时出错: {}", e);
    }
}
