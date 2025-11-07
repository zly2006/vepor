use crate::types::Point;
use crate::geometry::{distance, is_angle_in_arc};

/// Find intersection points between two line segments
pub fn line_line_intersection(l1_start: Point, l1_end: Point, l2_start: Point, l2_end: Point) -> Vec<Point> {
    let mut intersections = Vec::new();

    let x1 = l1_start.x;
    let y1 = l1_start.y;
    let x2 = l1_end.x;
    let y2 = l1_end.y;
    let x3 = l2_start.x;
    let y3 = l2_start.y;
    let x4 = l2_end.x;
    let y4 = l2_end.y;

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
pub fn line_arc_intersection(line_start: Point, line_end: Point, center: Point, radius: f64, start_angle: f64, end_angle: f64) -> Vec<Point> {
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
pub fn arc_arc_intersection(c1: Point, r1: f64, start1: f64, end1: f64, c2: Point, r2: f64, start2: f64, end2: f64) -> Vec<Point> {
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

    let h = if h_squared < 0.0 { 0.0 } else { h_squared.sqrt() };

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
            Point { x: 10.0, y: 0.0 }
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
            Point { x: 10.0, y: 5.0 }
        );
        assert_eq!(pts.len(), 0);
    }

    #[test]
    fn test_line_line_intersection_no_overlap() {
        let pts = line_line_intersection(
            Point { x: 0.0, y: 0.0 },
            Point { x: 2.0, y: 2.0 },
            Point { x: 5.0, y: 5.0 },
            Point { x: 10.0, y: 10.0 }
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
            360.0
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
            360.0
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
            360.0
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
            360.0
        );
        assert_eq!(pts.len(), 0);
    }
}

