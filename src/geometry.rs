use crate::types::{Point, PathSegment};

/// Calculate distance between two points
pub fn distance(p1: Point, p2: Point) -> f64 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

/// Check if a point is inside a circle
pub fn point_inside_circle(point: Point, center: Point, radius: f64) -> bool {
    distance(point, center) < radius
}

/// Check if a point is inside a rectangle
pub fn point_inside_rectangle(point: Point, top_left: Point, bottom_right: Point) -> bool {
    point.x >= top_left.x && point.x <= bottom_right.x &&
    point.y >= top_left.y && point.y <= bottom_right.y
}

/// Helper function to check if an angle is within an arc range
pub fn is_angle_in_arc(angle: f64, start_angle: f64, end_angle: f64) -> bool {
    // Normalize angles to [0, 360)
    let mut a = angle % 360.0;
    if a < 0.0 {
        a += 360.0;
    }

    let mut start = start_angle % 360.0;
    if start < 0.0 {
        start += 360.0;
    }

    let mut end = end_angle % 360.0;
    if end < 0.0 {
        end += 360.0;
    }

    // Handle full circle case
    if (end - start).abs() < 1e-10 || ((end - start).abs() - 360.0).abs() < 1e-10 {
        return true;
    }

    // Handle arc crossing 0 degrees
    if start <= end {
        a >= start - 1e-6 && a <= end + 1e-6
    } else {
        a >= start - 1e-6 || a <= end + 1e-6
    }
}

/// Get the starting point of a path segment list
pub fn get_starting_point(segments: &Vec<PathSegment>) -> Option<Point> {
    for segment in segments {
        match segment {
            PathSegment::Line(start, _) => return Some(*start),
            PathSegment::Arc(center, radius, start_angle, _) => {
                let rad = start_angle.to_radians();
                let start_point = Point {
                    x: center.x + radius * rad.cos(),
                    y: center.y + radius * rad.sin(),
                };
                return Some(start_point);
            }
            PathSegment::ConnectedArc(_, _, _, _, start_point, _) => return Some(*start_point),
            PathSegment::ClosePath => continue,
        }
    }
    None
}

/// Get midpoint of a segment for inside/outside testing
pub fn get_segment_midpoint(segment: &PathSegment) -> Point {
    match segment {
        PathSegment::Line(start, end) => Point {
            x: (start.x + end.x) / 2.0,
            y: (start.y + end.y) / 2.0,
        },
        PathSegment::Arc(center, radius, start_angle, end_angle) => {
            let mid_angle = (start_angle + end_angle) / 2.0;
            let rad = mid_angle.to_radians();
            Point {
                x: center.x + radius * rad.cos(),
                y: center.y + radius * rad.sin(),
            }
        }
        PathSegment::ConnectedArc(center, radius, start_angle, end_angle, _, _) => {
            let mid_angle = (start_angle + end_angle) / 2.0;
            let rad = mid_angle.to_radians();
            Point {
                x: center.x + radius * rad.cos(),
                y: center.y + radius * rad.sin(),
            }
        }
        PathSegment::ClosePath => Point { x: 0.0, y: 0.0 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        assert!((distance(p1, p2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_point_inside_circle() {
        let center = Point { x: 0.0, y: 0.0 };
        let radius = 5.0;
        
        assert!(point_inside_circle(Point { x: 0.0, y: 0.0 }, center, radius));
        assert!(point_inside_circle(Point { x: 3.0, y: 0.0 }, center, radius));
        assert!(!point_inside_circle(Point { x: 6.0, y: 0.0 }, center, radius));
    }

    #[test]
    fn test_point_inside_rectangle() {
        let top_left = Point { x: 0.0, y: 0.0 };
        let bottom_right = Point { x: 10.0, y: 10.0 };
        
        assert!(point_inside_rectangle(Point { x: 5.0, y: 5.0 }, top_left, bottom_right));
        assert!(point_inside_rectangle(Point { x: 0.0, y: 0.0 }, top_left, bottom_right));
        assert!(!point_inside_rectangle(Point { x: 11.0, y: 5.0 }, top_left, bottom_right));
    }

    #[test]
    fn test_is_angle_in_arc() {
        // Normal arc
        assert!(is_angle_in_arc(45.0, 0.0, 90.0));
        assert!(!is_angle_in_arc(100.0, 0.0, 90.0));
        
        // Arc crossing 0 degrees
        assert!(is_angle_in_arc(350.0, 340.0, 10.0));
        assert!(is_angle_in_arc(5.0, 340.0, 10.0));
        assert!(!is_angle_in_arc(180.0, 340.0, 10.0));
        
        // Full circle
        assert!(is_angle_in_arc(180.0, 0.0, 360.0));
    }

    #[test]
    fn test_get_segment_midpoint() {
        // Line segment
        let line = PathSegment::Line(
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 }
        );
        let mid = get_segment_midpoint(&line);
        assert!((mid.x - 5.0).abs() < 1e-10);
        assert!((mid.y - 5.0).abs() < 1e-10);
        
        // Arc segment
        let arc = PathSegment::Arc(
            Point { x: 0.0, y: 0.0 },
            10.0,
            0.0,
            90.0
        );
        let mid = get_segment_midpoint(&arc);
        // Midpoint should be at 45 degrees
        let expected_x = 10.0 * (45.0_f64.to_radians()).cos();
        let expected_y = 10.0 * (45.0_f64.to_radians()).sin();
        assert!((mid.x - expected_x).abs() < 1e-10);
        assert!((mid.y - expected_y).abs() < 1e-10);
    }
}

