use crate::types::{Point, PathSegment, ResolvedShape};
use crate::intersection::{line_line_intersection, line_arc_intersection, arc_arc_intersection};
use crate::geometry::{distance, get_segment_midpoint};

/// Find all intersection points between two resolved shapes
pub fn find_shape_intersections(shape1: &ResolvedShape, shape2: &ResolvedShape) -> Vec<Point> {
    let mut intersections = Vec::new();

    for seg1 in &shape1.segments {
        for seg2 in &shape2.segments {
            let pts = match (seg1, seg2) {
                (PathSegment::Line(s1, e1), PathSegment::Line(s2, e2)) => {
                    line_line_intersection(*s1, *e1, *s2, *e2)
                }
                (PathSegment::Line(s, e), PathSegment::Arc(c, r, start_angle, end_angle)) => {
                    line_arc_intersection(*s, *e, *c, *r, *start_angle, *end_angle)
                }
                (PathSegment::Arc(c, r, start_angle, end_angle), PathSegment::Line(s, e)) => {
                    line_arc_intersection(*s, *e, *c, *r, *start_angle, *end_angle)
                }
                (PathSegment::Arc(c1, r1, start1, end1), PathSegment::Arc(c2, r2, start2, end2)) => {
                    arc_arc_intersection(*c1, *r1, *start1, *end1, *c2, *r2, *start2, *end2)
                }
                (PathSegment::Line(s, e), PathSegment::ConnectedArc(c, r, start_angle, end_angle, _, _)) => {
                    line_arc_intersection(*s, *e, *c, *r, *start_angle, *end_angle)
                }
                (PathSegment::ConnectedArc(c, r, start_angle, end_angle, _, _), PathSegment::Line(s, e)) => {
                    line_arc_intersection(*s, *e, *c, *r, *start_angle, *end_angle)
                }
                (PathSegment::Arc(c1, r1, start1, end1), PathSegment::ConnectedArc(c2, r2, start2, end2, _, _)) => {
                    arc_arc_intersection(*c1, *r1, *start1, *end1, *c2, *r2, *start2, *end2)
                }
                (PathSegment::ConnectedArc(c1, r1, start1, end1, _, _), PathSegment::Arc(c2, r2, start2, end2)) => {
                    arc_arc_intersection(*c1, *r1, *start1, *end1, *c2, *r2, *start2, *end2)
                }
                (PathSegment::ConnectedArc(c1, r1, start1, end1, _, _), PathSegment::ConnectedArc(c2, r2, start2, end2, _, _)) => {
                    arc_arc_intersection(*c1, *r1, *start1, *end1, *c2, *r2, *start2, *end2)
                }
                _ => Vec::new(),
            };
            intersections.extend(pts);
        }
    }

    intersections
}

/// Check if a point is inside a shape using ray casting algorithm
pub fn point_inside_shape(point: Point, shape: &ResolvedShape) -> bool {
    let ray_end = Point { x: point.x + 10000.0, y: point.y };
    let mut intersection_count = 0;

    for segment in &shape.segments {
        match segment {
            PathSegment::Line(start, end) => {
                if !line_line_intersection(point, ray_end, *start, *end).is_empty() {
                    intersection_count += 1;
                }
            }
            PathSegment::Arc(center, radius, start_angle, end_angle) => {
                let arc_intersections = line_arc_intersection(point, ray_end, *center, *radius, *start_angle, *end_angle);
                intersection_count += arc_intersections.len();

                // For full circles, also check if point is inside
                if (*end_angle - *start_angle).abs() >= 360.0 - 1e-6 {
                    if distance(point, *center) < *radius - 1e-10 {
                        return true;
                    }
                }
            }
            PathSegment::ConnectedArc(center, radius, start_angle, end_angle, _, _) => {
                let arc_intersections = line_arc_intersection(point, ray_end, *center, *radius, *start_angle, *end_angle);
                intersection_count += arc_intersections.len();
            }
            PathSegment::ClosePath => {}
        }
    }

    intersection_count % 2 == 1
}

/// Compute union of two shapes
pub fn compute_union(shape1: &ResolvedShape, shape2: &ResolvedShape, intersections: &Vec<Point>) -> ResolvedShape {
    let mut result_segments = Vec::new();

    // Split segments at intersection points and keep outer boundaries
    for segment in &shape1.segments {
        if !matches!(segment, PathSegment::ClosePath) {
            let midpoint = get_segment_midpoint(segment);
            // Keep segment if it's outside shape2 or on the boundary
            if !point_inside_shape(midpoint, shape2) {
                result_segments.push(*segment);
            }
        }
    }

    for segment in &shape2.segments {
        if !matches!(segment, PathSegment::ClosePath) {
            let midpoint = get_segment_midpoint(segment);
            // Keep segment if it's outside shape1
            if !point_inside_shape(midpoint, shape1) {
                result_segments.push(*segment);
            }
        }
    }

    // Add connecting arcs at intersection points
    for &intersection in intersections {
        result_segments.push(PathSegment::ConnectedArc(
            Point { x: 0.0, y: 0.0 },
            0.0,
            0.0,
            0.0,
            intersection,
            intersection,
        ));
    }

    if !result_segments.is_empty() {
        result_segments.push(PathSegment::ClosePath);
    }

    ResolvedShape { segments: result_segments }
}

/// Compute subtraction of two shapes
pub fn compute_subtract(shape1: &ResolvedShape, shape2: &ResolvedShape, _intersections: &Vec<Point>) -> ResolvedShape {
    let mut result_segments = Vec::new();

    // Keep segments from shape1 that are outside shape2
    for segment in &shape1.segments {
        if !matches!(segment, PathSegment::ClosePath) {
            let midpoint = get_segment_midpoint(segment);
            if !point_inside_shape(midpoint, shape2) {
                result_segments.push(*segment);
            }
        }
    }

    // Add reversed segments from shape2 that are inside shape1 (creates a hole)
    for segment in &shape2.segments {
        if !matches!(segment, PathSegment::ClosePath) {
            let midpoint = get_segment_midpoint(segment);
            if point_inside_shape(midpoint, shape1) {
                // Reverse the segment
                let reversed = match segment {
                    PathSegment::Line(s, e) => PathSegment::Line(*e, *s),
                    PathSegment::Arc(c, r, sa, ea) => PathSegment::Arc(*c, *r, *ea, *sa),
                    _ => *segment,
                };
                result_segments.push(reversed);
            }
        }
    }

    if !result_segments.is_empty() {
        result_segments.push(PathSegment::ClosePath);
    }

    ResolvedShape { segments: result_segments }
}

/// Compute XOR of two shapes
pub fn compute_xor(shape1: &ResolvedShape, shape2: &ResolvedShape, _intersections: &Vec<Point>) -> ResolvedShape {
    let mut result_segments = Vec::new();

    // Keep segments from shape1 that are outside shape2
    for segment in &shape1.segments {
        if !matches!(segment, PathSegment::ClosePath) {
            let midpoint = get_segment_midpoint(segment);
            if !point_inside_shape(midpoint, shape2) {
                result_segments.push(*segment);
            }
        }
    }

    // Keep segments from shape2 that are outside shape1
    for segment in &shape2.segments {
        if !matches!(segment, PathSegment::ClosePath) {
            let midpoint = get_segment_midpoint(segment);
            if !point_inside_shape(midpoint, shape1) {
                result_segments.push(*segment);
            }
        }
    }

    if !result_segments.is_empty() {
        result_segments.push(PathSegment::ClosePath);
    }

    ResolvedShape { segments: result_segments }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Shape};
    use crate::resolver::resolve_shape;

    #[test]
    fn test_find_shape_intersections_circle_rectangle() {
        let circle = resolve_shape(&Shape::Circle {
            center: Point { x: 10.0, y: 10.0 },
            radius: 5.0,
        });
        let rectangle = resolve_shape(&Shape::Rectangle {
            top_left: Point { x: 8.0, y: 8.0 },
            bottom_right: Point { x: 15.0, y: 12.0 },
        });
        
        let intersections = find_shape_intersections(&circle, &rectangle);
        assert_eq!(intersections.len(), 3);
    }

    #[test]
    fn test_point_inside_shape_circle() {
        let circle = resolve_shape(&Shape::Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 5.0,
        });
        
        assert!(point_inside_shape(Point { x: 0.0, y: 0.0 }, &circle));
        assert!(point_inside_shape(Point { x: 3.0, y: 0.0 }, &circle));
        assert!(!point_inside_shape(Point { x: 10.0, y: 0.0 }, &circle));
    }

    #[test]
    fn test_point_inside_shape_rectangle() {
        let rectangle = resolve_shape(&Shape::Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 10.0, y: 10.0 },
        });
        
        assert!(point_inside_shape(Point { x: 5.0, y: 5.0 }, &rectangle));
        assert!(!point_inside_shape(Point { x: 15.0, y: 5.0 }, &rectangle));
    }
}

