mod types;
mod geometry;
mod intersection;
mod boolean_ops;
mod resolver;

use types::{Point, Shape};
use resolver::resolve_shape;
use boolean_ops::{find_shape_intersections, compute_area, compute_signed_area, is_shape_counter_clockwise};
use crate::geometry::signed_area_of_path;
use crate::intersection::line_line_intersection;
use crate::types::PathSegment;

fn main() {
    let radius = 5.0;
    let center = Point { x: 1.0, y: 3.0 };

    // Use single full circle arc for comparison
    let segments = vec![
        PathSegment::Arc(center, radius, 0.0, 360.0),
    ];

    let area_full = signed_area_of_path(&segments);

    // Now try with multiple arcs
    let segments_multi = vec![
        PathSegment::Arc(center, radius, 0.0, 70.0),
        PathSegment::Arc(center, radius, 70.0, 160.0),
        PathSegment::Arc(center, radius, 160.0, 270.0),
        PathSegment::Arc(center, radius, 270.0, 360.0),
    ];

    let area_multi = signed_area_of_path(&segments_multi);
    println!("Area with single full arc: {}", area_full);
    println!("Area with multiple arcs: {}", area_multi);
    println!("25 pi is approximately: {}", 25.0 * std::f64::consts::PI);
}

