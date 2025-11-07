use crate::geometry::{distance, is_angle_in_arc};
use crate::types::Point;

/// Find intersection points between two line segments
pub fn line_line_intersection(
    l1_start: Point,
    l1_end: Point,
    l2_start: Point,
    l2_end: Point,
) -> Vec<Point> {
    let mut intersections = Vec::new();

    let x1 = l1_start.x;
    let y1 = l1_start.y;
    let x2 = l1_end.x;
    let y2 = l1_end.y;
    let x3 = l2_start.x;
    let y3 = l2_start.y;
    let x4 = l2_end.x;
    let y4 = l2_end.y;

    // Check if either segment is actually a point
    let l1_is_point = (x1 - x2).abs() < 1e-10 && (y1 - y2).abs() < 1e-10;
    let l2_is_point = (x3 - x4).abs() < 1e-10 && (y3 - y4).abs() < 1e-10;

    if l1_is_point && l2_is_point {
        // Both are points - check if they're the same
        if (x1 - x3).abs() < 1e-10 && (y1 - y3).abs() < 1e-10 {
            intersections.push(Point { x: x1, y: y1 });
        }
        return intersections;
    }

    if l1_is_point {
        // Check if point l1 is on line segment l2
        let t = if (x4 - x3).abs() > (y4 - y3).abs() {
            (x1 - x3) / (x4 - x3)
        } else {
            (y1 - y3) / (y4 - y3)
        };
        if t >= -1e-10 && t <= 1.0 + 1e-10 {
            let px = x3 + t * (x4 - x3);
            let py = y3 + t * (y4 - y3);
            if (px - x1).abs() < 1e-10 && (py - y1).abs() < 1e-10 {
                intersections.push(Point { x: x1, y: y1 });
            }
        }
        return intersections;
    }

    if l2_is_point {
        // Check if point l2 is on line segment l1
        let t = if (x2 - x1).abs() > (y2 - y1).abs() {
            (x3 - x1) / (x2 - x1)
        } else {
            (y3 - y1) / (y2 - y1)
        };
        if t >= -1e-10 && t <= 1.0 + 1e-10 {
            let px = x1 + t * (x2 - x1);
            let py = y1 + t * (y2 - y1);
            if (px - x3).abs() < 1e-10 && (py - y3).abs() < 1e-10 {
                intersections.push(Point { x: x3, y: y3 });
            }
        }
        return intersections;
    }

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if denom.abs() < 1e-10 {
        return intersections; // Lines are parallel or coincident
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denom;

    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        let ix = x1 + t * (x2 - x1);
        let iy = y1 + t * (y2 - y1);
        intersections.push(Point { x: ix, y: iy });
    }

    intersections
}

/// Find intersection points between a line segment and a circle arc
pub fn line_arc_intersection(
    line_start: Point,
    line_end: Point,
    center: Point,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> Vec<Point> {
    let mut intersections = Vec::new();

    let dx = line_end.x - line_start.x;
    let dy = line_end.y - line_start.y;
    let fx = line_start.x - center.x;
    let fy = line_start.y - center.y;

    let a = dx * dx + dy * dy;

    // Handle degenerate case: line segment is a point
    if a < 1e-10 {
        let dist = (fx * fx + fy * fy).sqrt();
        if (dist - radius).abs() < 1e-10 {
            let angle = fy.atan2(fx).to_degrees();
            if is_angle_in_arc(angle, start_angle, end_angle) {
                intersections.push(line_start);
            }
        }
        return intersections;
    }

    let b = 2.0 * (fx * dx + fy * dy);
    let c = fx * fx + fy * fy - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return intersections; // No intersection
    }

    let sqrt_disc = discriminant.sqrt();
    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);

    // Check first intersection point
    if t1 >= -1e-10 && t1 <= 1.0 + 1e-10 {
        let ix = line_start.x + t1 * dx;
        let iy = line_start.y + t1 * dy;
        let pt = Point { x: ix, y: iy };

        // Check if point is on the arc (within start_angle and end_angle)
        let angle = (iy - center.y).atan2(ix - center.x).to_degrees();
        if is_angle_in_arc(angle, start_angle, end_angle) {
            intersections.push(pt);
        }
    }

    // Check second intersection point
    if discriminant > 1e-10 && t2 >= -1e-10 && t2 <= 1.0 + 1e-10 {
        let ix = line_start.x + t2 * dx;
        let iy = line_start.y + t2 * dy;
        let pt = Point { x: ix, y: iy };

        // Check if point is on the arc
        let angle = (iy - center.y).atan2(ix - center.x).to_degrees();
        if is_angle_in_arc(angle, start_angle, end_angle) {
            intersections.push(pt);
        }
    }

    intersections
}

/// Find intersection points between two circle arcs
pub fn arc_arc_intersection(
    c1: Point,
    r1: f64,
    start1: f64,
    end1: f64,
    c2: Point,
    r2: f64,
    start2: f64,
    end2: f64,
) -> Vec<Point> {
    let mut intersections = Vec::new();

    let d = distance(c1, c2);

    // No intersection if circles are too far apart or one contains the other
    if d > r1 + r2 + 1e-10 || d < (r1 - r2).abs() - 1e-10 {
        return intersections;
    }

    // Circles are coincident
    if d < 1e-10 {
        if (r1 - r2).abs() < 1e-10 {
            // Same circle - infinite intersections, return empty for now
            return intersections;
        }
        return intersections;
    }

    // Circles are tangent (one intersection point)
    if (d - (r1 + r2)).abs() < 1e-10 || (d - (r1 - r2).abs()).abs() < 1e-10 {
        let t = r1 / d;
        let px = c1.x + t * (c2.x - c1.x);
        let py = c1.y + t * (c2.y - c1.y);
        let pt = Point { x: px, y: py };

        let angle1 = (py - c1.y).atan2(px - c1.x).to_degrees();
        let angle2 = (py - c2.y).atan2(px - c2.x).to_degrees();

        if is_angle_in_arc(angle1, start1, end1) && is_angle_in_arc(angle2, start2, end2) {
            intersections.push(pt);
        }
        return intersections;
    }

    // Two intersection points
    let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
    let h_squared = r1 * r1 - a * a;

    if h_squared < -1e-10 {
        return intersections; // No real intersection
    }

    let h = if h_squared < 0.0 {
        0.0
    } else {
        h_squared.sqrt()
    };

    let px = c1.x + a * (c2.x - c1.x) / d;
    let py = c1.y + a * (c2.y - c1.y) / d;

    let ix1 = px + h * (c2.y - c1.y) / d;
    let iy1 = py - h * (c2.x - c1.x) / d;
    let pt1 = Point { x: ix1, y: iy1 };

    let angle1_1 = (iy1 - c1.y).atan2(ix1 - c1.x).to_degrees();
    let angle2_1 = (iy1 - c2.y).atan2(ix1 - c2.x).to_degrees();

    if is_angle_in_arc(angle1_1, start1, end1) && is_angle_in_arc(angle2_1, start2, end2) {
        intersections.push(pt1);
    }

    if h > 1e-10 {
        let ix2 = px - h * (c2.y - c1.y) / d;
        let iy2 = py + h * (c2.x - c1.x) / d;
        let pt2 = Point { x: ix2, y: iy2 };

        let angle1_2 = (iy2 - c1.y).atan2(ix2 - c1.x).to_degrees();
        let angle2_2 = (iy2 - c2.y).atan2(ix2 - c2.x).to_degrees();

        if is_angle_in_arc(angle1_2, start1, end1) && is_angle_in_arc(angle2_2, start2, end2) {
            intersections.push(pt2);
        }
    }

    intersections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_line_intersection_normal() {
        let pts = line_line_intersection(
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 10.0 },
            Point { x: 10.0, y: 0.0 },
        );
        assert_eq!(pts.len(), 1);
        assert!((pts[0].x - 5.0).abs() < 1e-10);
        assert!((pts[0].y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_line_line_intersection_parallel() {
        let pts = line_line_intersection(
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 0.0, y: 5.0 },
            Point { x: 10.0, y: 5.0 },
        );
        assert_eq!(pts.len(), 0);
    }

    #[test]
    fn test_line_line_intersection_no_overlap() {
        let pts = line_line_intersection(
            Point { x: 0.0, y: 0.0 },
            Point { x: 2.0, y: 2.0 },
            Point { x: 5.0, y: 5.0 },
            Point { x: 10.0, y: 10.0 },
        );
        assert_eq!(pts.len(), 0);
    }

    #[test]
    fn test_line_arc_intersection() {
        let pts = line_arc_intersection(
            Point { x: -10.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 0.0, y: 0.0 },
            5.0,
            0.0,
            360.0,
        );
        assert_eq!(pts.len(), 2);
    }

    #[test]
    fn test_arc_arc_intersection_two_points() {
        let pts = arc_arc_intersection(
            Point { x: 0.0, y: 0.0 },
            5.0,
            0.0,
            360.0,
            Point { x: 6.0, y: 0.0 },
            5.0,
            0.0,
            360.0,
        );
        assert_eq!(pts.len(), 2);
    }

    #[test]
    fn test_arc_arc_intersection_tangent() {
        let pts = arc_arc_intersection(
            Point { x: 0.0, y: 0.0 },
            3.0,
            0.0,
            360.0,
            Point { x: 6.0, y: 0.0 },
            3.0,
            0.0,
            360.0,
        );
        assert_eq!(pts.len(), 1);
        assert!((pts[0].x - 3.0).abs() < 1e-10);
        assert!(pts[0].y.abs() < 1e-10);
    }

    #[test]
    fn test_arc_arc_intersection_no_overlap() {
        let pts = arc_arc_intersection(
            Point { x: 0.0, y: 0.0 },
            2.0,
            0.0,
            360.0,
            Point { x: 10.0, y: 0.0 },
            2.0,
            0.0,
            360.0,
        );
        assert_eq!(pts.len(), 0);
    }

    // Edge case tests: Endpoint intersections

    #[test]
    fn test_line_line_endpoint_intersection() {
        // Two line segments meeting at endpoint (0,0)
        let pts = line_line_intersection(
            Point { x: -5.0, y: 0.0 },
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.0, y: 0.0 },
            Point { x: 5.0, y: 5.0 },
        );
        assert_eq!(pts.len(), 1, "Should find endpoint intersection");
        assert!((pts[0].x - 0.0).abs() < 1e-10);
        assert!((pts[0].y - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_line_line_endpoint_on_segment() {
        // Endpoint of line1 is on the middle of line2
        let pts = line_line_intersection(
            Point { x: 0.0, y: 5.0 },
            Point { x: 5.0, y: 5.0 },
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.0, y: 10.0 },
        );
        assert_eq!(pts.len(), 1, "Should find intersection at endpoint");
        assert!((pts[0].x - 0.0).abs() < 1e-10);
        assert!((pts[0].y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_line_line_t_intersection() {
        // T-shaped intersection where one line ends at the middle of another
        let pts = line_line_intersection(
            Point { x: 5.0, y: 0.0 },
            Point { x: 5.0, y: 5.0 },
            Point { x: 0.0, y: 5.0 },
            Point { x: 10.0, y: 5.0 },
        );
        assert_eq!(pts.len(), 1, "Should find T-intersection");
        assert!((pts[0].x - 5.0).abs() < 1e-10);
        assert!((pts[0].y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_line_arc_endpoint_on_arc() {
        // Line segment with endpoint exactly on the arc
        // Circle centered at (5, 0) with radius 5
        // Arc from 90° to 270° (left semicircle)
        // Line from (0, 0) to (5, 5)
        let pts = line_arc_intersection(
            Point { x: 0.0, y: 0.0 },
            Point { x: 5.0, y: 5.0 },
            Point { x: 5.0, y: 0.0 },
            5.0,
            90.0,
            270.0,
        );
        // The line actually intersects at two points: (0, 0) and (5, 5)
        // (0, 0) is at 180° from center (5, 0), which is in [90, 270]
        // (5, 5) is at 90° from center (5, 0), which is in [90, 270]
        assert_eq!(pts.len(), 2, "Should find both intersections");
    }

    #[test]
    fn test_line_arc_tangent_at_endpoint() {
        // Line tangent to circle at its endpoint
        // Circle at origin with radius 5
        // Horizontal line at y=5 from x=-10 to x=0
        let pts = line_arc_intersection(
            Point { x: -10.0, y: 5.0 },
            Point { x: 0.0, y: 5.0 },
            Point { x: 0.0, y: 0.0 },
            5.0,
            0.0,
            180.0, // Top half of circle
        );
        assert_eq!(pts.len(), 1, "Should find tangent point at endpoint");
        assert!((pts[0].x - 0.0).abs() < 1e-10);
        assert!((pts[0].y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_line_arc_passes_through_arc_endpoints() {
        // Line passing through both endpoints of an arc
        // Arc from (3, 4) to (4, 3) on circle centered at origin with radius 5
        // This arc goes from ~53.13° to ~36.87°
        let start_angle = (4.0_f64).atan2(3.0).to_degrees();
        let end_angle = (3.0_f64).atan2(4.0).to_degrees();

        let pts = line_arc_intersection(
            Point { x: 0.0, y: 7.0 },
            Point { x: 7.0, y: 0.0 },
            Point { x: 0.0, y: 0.0 },
            5.0,
            end_angle,
            start_angle,
        );
        assert_eq!(pts.len(), 2, "Should find both arc endpoints");
    }

    #[test]
    fn test_arc_arc_endpoint_touches() {
        // Two arcs where endpoints touch
        // Arc1: centered at (0, 0), radius 5, from 0° to 90°
        //   - starts at (5, 0), ends at (0, 5)
        // Arc2: centered at (5, 5), radius 5, from 180° to 270°
        //   - starts at (0, 5), ends at (5, 0)
        let pts = arc_arc_intersection(
            Point { x: 0.0, y: 0.0 },
            5.0,
            0.0,
            90.0,
            Point { x: 5.0, y: 5.0 },
            5.0,
            180.0,
            270.0,
        );
        // These two circles intersect at (0, 5) and (5, 0)
        // Both points should be on both arcs
        assert_eq!(pts.len(), 2, "Should find both intersection points");
        // Verify the points
        let has_5_0 = pts
            .iter()
            .any(|p| (p.x - 5.0).abs() < 1e-10 && p.y.abs() < 1e-10);
        let has_0_5 = pts
            .iter()
            .any(|p| p.x.abs() < 1e-10 && (p.y - 5.0).abs() < 1e-10);
        assert!(has_5_0 && has_0_5, "Should have both (5,0) and (0,5)");
    }

    #[test]
    fn test_arc_arc_partial_overlap() {
        // Two arcs that intersect at two points
        // Arc1: center (0, 0), radius 5, from 0° to 90° (first quadrant)
        // Arc2: center (6, 0), radius 5, from 90° to 180° (second quadrant relative to its center)
        // These circles intersect at two points with y > 0 and y < 0
        let pts = arc_arc_intersection(
            Point { x: 0.0, y: 0.0 },
            5.0,
            -60.0,
            60.0,
            Point { x: 6.0, y: 0.0 },
            5.0,
            120.0,
            240.0,
        );
        assert_eq!(pts.len(), 2, "Should find two intersection points");
        // Verify both points are symmetric about x-axis
        assert!(
            (pts[0].y + pts[1].y).abs() < 1e-10,
            "Points should be symmetric about x-axis"
        );
        assert!(
            pts[0].y.abs() > 0.1 && pts[1].y.abs() > 0.1,
            "Points should have non-zero y"
        );
    }

    #[test]
    fn test_arc_arc_one_endpoint_on_other_arc() {
        // Arc1 endpoint lies on Arc2 (but Arc2 endpoint doesn't lie on Arc1)
        // Arc1: center (0, 0), radius 5, from 0° to 90°
        //   - starts at (5, 0)
        //   - ends at (0, 5)
        // Arc2: center (0, 0), radius 5, from 45° to 180°
        //   - includes the point (0, 5) which is Arc1's endpoint
        let pts = arc_arc_intersection(
            Point { x: 0.0, y: 0.0 },
            5.0,
            0.0,
            90.0,
            Point { x: 0.0, y: 0.0 },
            5.0,
            45.0,
            180.0,
        );
        // Same circle, overlapping arcs - intersection is the overlapping portion
        // Since they share the same circle, intersection at endpoints/overlap
        assert!(
            pts.len() == 0,
            "Same circle arcs return empty (infinite overlap)"
        );
    }

    #[test]
    fn test_line_segment_as_point() {
        // Degenerate case: line segment is actually a point
        let pts = line_line_intersection(
            Point { x: 5.0, y: 5.0 },
            Point { x: 5.0, y: 5.0 },
            Point { x: 0.0, y: 5.0 },
            Point { x: 10.0, y: 5.0 },
        );
        assert_eq!(pts.len(), 1, "Point on line should be detected");
        assert!((pts[0].x - 5.0).abs() < 1e-10);
        assert!((pts[0].y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_line_arc_line_passes_through_center() {
        // Line passes through the center of the circle
        let pts = line_arc_intersection(
            Point { x: -10.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 0.0, y: 0.0 },
            5.0,
            0.0,
            360.0,
        );
        assert_eq!(pts.len(), 2, "Line through center intersects at two points");
        // Should intersect at (-5, 0) and (5, 0)
        let x_coords: Vec<f64> = pts.iter().map(|p| p.x).collect();
        assert!(x_coords.contains(&5.0) || (x_coords[0] - 5.0).abs() < 1e-10);
        assert!(x_coords.contains(&-5.0) || (x_coords[1] + 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_arc_with_small_angle_range() {
        // Very small arc (1 degree) and line intersection
        let pts = line_arc_intersection(
            Point { x: 0.0, y: -10.0 },
            Point { x: 0.0, y: 10.0 },
            Point { x: 0.0, y: 0.0 },
            5.0,
            89.0, // Very small arc near 90°
            91.0,
        );
        assert_eq!(pts.len(), 1, "Should intersect small arc");
        assert!((pts[0].y - 5.0).abs() < 0.1, "Should be near (0, 5)");
    }

    #[test]
    fn test_collinear_overlapping_segments() {
        // Two collinear segments that overlap
        let pts = line_line_intersection(
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 5.0, y: 0.0 },
            Point { x: 15.0, y: 0.0 },
        );
        // Current implementation returns empty for collinear (treated as parallel)
        // This is acceptable as there are infinite intersection points
        assert_eq!(pts.len(), 0, "Collinear segments return empty");
    }
}
