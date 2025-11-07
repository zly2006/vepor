use crate::types::{Shape, Point, PathSegment, ResolvedShape};
use crate::geometry::get_starting_point;
use crate::boolean_ops::{find_shape_intersections, compute_union, compute_subtract, compute_xor};

pub fn resolve_shape(shape: &Shape) -> ResolvedShape {
    match shape {
        Shape::Circle { center, radius } => {
            ResolvedShape {
                segments: vec![PathSegment::Arc(
                    *center,
                    *radius,
                    0.0,
                    360.0,
                )],
            }
        }
        Shape::Rectangle { top_left, bottom_right } => {
            let top_right = Point { x: bottom_right.x, y: top_left.y };
            let bottom_left = Point { x: top_left.x, y: bottom_right.y };
            ResolvedShape {
                segments: vec![
                    PathSegment::Line(*top_left, top_right),
                    PathSegment::Line(top_right, *bottom_right),
                    PathSegment::Line(*bottom_right, bottom_left),
                    PathSegment::Line(bottom_left, *top_left),
                    PathSegment::ClosePath,
                ],
            }
        }
        Shape::Scale(shape, factor) => {
            let scale_center = get_starting_point(&resolve_shape(shape).segments).unwrap_or(Point { x: 0.0, y: 0.0 });
            let mut resolved = resolve_shape(shape);
            for segment in &mut resolved.segments {
                match segment {
                    PathSegment::Line(start, end) => {
                        start.x = scale_center.x + (*factor) * (start.x - scale_center.x);
                        start.y = scale_center.y + (*factor) * (start.y - scale_center.y);
                        end.x = scale_center.x + (*factor) * (end.x - scale_center.x);
                        end.y = scale_center.y + (*factor) * (end.y - scale_center.y);
                    }
                    PathSegment::Arc(center, radius, _, _) => {
                        center.x = scale_center.x + (*factor) * (center.x - scale_center.x);
                        center.y = scale_center.y + (*factor) * (center.y - scale_center.y);
                        *radius *= *factor;
                    }
                    PathSegment::ConnectedArc(center, radius, _, _, start_pt, end_pt) => {
                        center.x = scale_center.x + (*factor) * (center.x - scale_center.x);
                        center.y = scale_center.y + (*factor) * (center.y - scale_center.y);
                        *radius *= *factor;
                        start_pt.x = scale_center.x + (*factor) * (start_pt.x - scale_center.x);
                        start_pt.y = scale_center.y + (*factor) * (start_pt.y - scale_center.y);
                        end_pt.x = scale_center.x + (*factor) * (end_pt.x - scale_center.x);
                        end_pt.y = scale_center.y + (*factor) * (end_pt.y - scale_center.y);
                    }
                    PathSegment::ClosePath => {}
                    PathSegment::DrawPoint(point) => {
                        point.x = scale_center.x + (*factor) * (point.x - scale_center.x);
                        point.y = scale_center.y + (*factor) * (point.y - scale_center.y);
                    }
                }
            }
            resolved
        }
        Shape::Union(shape1, shape2) => {
            let resolved1 = resolve_shape(shape1);
            let resolved2 = resolve_shape(shape2);
            let intersections = find_shape_intersections(&resolved1, &resolved2);
            compute_union(&resolved1, &resolved2, &intersections)
        }
        Shape::Subtract(shape1, shape2) => {
            let resolved1 = resolve_shape(shape1);
            let resolved2 = resolve_shape(shape2);
            let intersections = find_shape_intersections(&resolved1, &resolved2);
            compute_subtract(&resolved1, &resolved2, &intersections)
        }
        Shape::Xor(shape1, shape2) => {
            let resolved1 = resolve_shape(shape1);
            let resolved2 = resolve_shape(shape2);
            let intersections = find_shape_intersections(&resolved1, &resolved2);
            compute_xor(&resolved1, &resolved2, &intersections)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_circle() {
        let circle = Shape::Circle {
            center: Point { x: 10.0, y: 10.0 },
            radius: 5.0,
        };
        let resolved = resolve_shape(&circle);
        assert_eq!(resolved.segments.len(), 1);
        match resolved.segments[0] {
            PathSegment::Arc(center, radius, start, end) => {
                assert_eq!(center.x, 10.0);
                assert_eq!(center.y, 10.0);
                assert_eq!(radius, 5.0);
                assert_eq!(start, 0.0);
                assert_eq!(end, 360.0);
            }
            _ => panic!("Expected Arc segment"),
        }
    }

    #[test]
    fn test_resolve_rectangle() {
        let rectangle = Shape::Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 10.0, y: 10.0 },
        };
        let resolved = resolve_shape(&rectangle);
        // 4 line segments + 1 ClosePath
        assert_eq!(resolved.segments.len(), 5);
    }

    #[test]
    fn test_resolve_scale() {
        let circle = Shape::Circle {
            center: Point { x: 10.0, y: 10.0 },
            radius: 5.0,
        };
        let scaled = Shape::Scale(Box::new(circle), 2.0);
        let resolved = resolve_shape(&scaled);

        match resolved.segments[0] {
            PathSegment::Arc(center, radius, _, _) => {
                assert_eq!(radius, 10.0); // Should be doubled
            }
            _ => panic!("Expected Arc segment"),
        }
    }

    #[test]
    fn test_resolve_union() {
        let circle = Shape::Circle {
            center: Point { x: 10.0, y: 10.0 },
            radius: 5.0,
        };
        let rectangle = Shape::Rectangle {
            top_left: Point { x: 8.0, y: 8.0 },
            bottom_right: Point { x: 15.0, y: 12.0 },
        };
        let union = Shape::Union(Box::new(circle), Box::new(rectangle));
        let resolved = resolve_shape(&union);

        // Should have segments from both shapes
        assert!(resolved.segments.len() > 0);
    }
}

