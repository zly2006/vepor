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

/// Calculate signed area of a closed path
/// Positive area = counter-clockwise orientation
/// Negative area = clockwise orientation
pub fn signed_area_of_path(segments: &Vec<PathSegment>) -> f64 {
    let mut area = 0.0;
    let mut current_point = match get_starting_point(segments) {
        Some(p) => p,
        None => return 0.0,
    };

    for segment in segments {
        match segment {
            PathSegment::Line(start, end) => {
                // Shoelace formula for line segment
                area += (start.x * end.y - end.x * start.y) / 2.0;
                current_point = *end;
            }
            PathSegment::Arc(center, radius, start_angle, end_angle) => {
                // Calculate arc contribution using sector formula
                let start_rad = start_angle.to_radians();
                let end_rad = end_angle.to_radians();

                let start_point = Point {
                    x: center.x + radius * start_rad.cos(),
                    y: center.y + radius * start_rad.sin(),
                };
                let end_point = Point {
                    x: center.x + radius * end_rad.cos(),
                    y: center.y + radius * end_rad.sin(),
                };

                // Area of circular sector
                let mut angle_diff = end_angle - start_angle;
                // Normalize to [-360, 360]
                while angle_diff > 360.0 {
                    angle_diff -= 360.0;
                }
                while angle_diff < -360.0 {
                    angle_diff += 360.0;
                }

                let sector_area = 0.5 * radius * radius * angle_diff.to_radians();

                // Add triangle area from origin to chord
                let chord_area = (center.x * (end_point.y - start_point.y) +
                                 start_point.x * (center.y - end_point.y) +
                                 end_point.x * (start_point.y - center.y)) / 2.0;

                area += sector_area + chord_area;
                current_point = end_point;
            }
            PathSegment::ConnectedArc(center, radius, start_angle, end_angle, start_point, end_point) => {
                // Similar to Arc but using provided points
                let mut angle_diff = end_angle - start_angle;
                while angle_diff > 360.0 {
                    angle_diff -= 360.0;
                }
                while angle_diff < -360.0 {
                    angle_diff += 360.0;
                }

                let sector_area = 0.5 * radius * radius * angle_diff.to_radians();

                let chord_area = (center.x * (end_point.y - start_point.y) +
                                 start_point.x * (center.y - end_point.y) +
                                 end_point.x * (start_point.y - center.y)) / 2.0;

                area += sector_area + chord_area;
                current_point = *end_point;
            }
            PathSegment::ClosePath => {
                // Close path connects last point to first
                if let Some(first_point) = get_starting_point(segments) {
                    area += (current_point.x * first_point.y - first_point.x * current_point.y) / 2.0;
                }
            }
        }
    }

    area
}

/// Calculate the absolute area of a closed path
pub fn area_of_path(segments: &Vec<PathSegment>) -> f64 {
    signed_area_of_path(segments).abs()
}

/// Check if a path is counter-clockwise oriented
pub fn is_counter_clockwise(segments: &Vec<PathSegment>) -> bool {
    signed_area_of_path(segments) > 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    // ...existing code...

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

    #[test]
    fn test_signed_area_square_ccw() {
        // Counter-clockwise square: (0,0) -> (10,0) -> (10,10) -> (0,10) -> close
        let segments = vec![
            PathSegment::Line(Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 0.0 }),
            PathSegment::Line(Point { x: 10.0, y: 0.0 }, Point { x: 10.0, y: 10.0 }),
            PathSegment::Line(Point { x: 10.0, y: 10.0 }, Point { x: 0.0, y: 10.0 }),
            PathSegment::Line(Point { x: 0.0, y: 10.0 }, Point { x: 0.0, y: 0.0 }),
            PathSegment::ClosePath,
        ];
        let area = signed_area_of_path(&segments);
        assert!((area - 100.0).abs() < 1e-10, "CCW square should have positive area");
        assert!(is_counter_clockwise(&segments));
    }

    #[test]
    fn test_signed_area_square_cw() {
        // Clockwise square: (0,0) -> (0,10) -> (10,10) -> (10,0) -> close
        let segments = vec![
            PathSegment::Line(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 10.0 }),
            PathSegment::Line(Point { x: 0.0, y: 10.0 }, Point { x: 10.0, y: 10.0 }),
            PathSegment::Line(Point { x: 10.0, y: 10.0 }, Point { x: 10.0, y: 0.0 }),
            PathSegment::Line(Point { x: 10.0, y: 0.0 }, Point { x: 0.0, y: 0.0 }),
            PathSegment::ClosePath,
        ];
        let area = signed_area_of_path(&segments);
        assert!((area + 100.0).abs() < 1e-10, "CW square should have negative area");
        assert!(!is_counter_clockwise(&segments));
    }

    #[test]
    fn test_area_of_path() {
        let segments = vec![
            PathSegment::Line(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 10.0 }),
            PathSegment::Line(Point { x: 0.0, y: 10.0 }, Point { x: 10.0, y: 10.0 }),
            PathSegment::Line(Point { x: 10.0, y: 10.0 }, Point { x: 10.0, y: 0.0 }),
            PathSegment::Line(Point { x: 10.0, y: 0.0 }, Point { x: 0.0, y: 0.0 }),
            PathSegment::ClosePath,
        ];
        let area = area_of_path(&segments);
        assert!((area - 100.0).abs() < 1e-10, "Absolute area should be 100");
    }

    #[test]
    fn test_signed_area_triangle() {
        // CCW triangle: (0,0) -> (10,0) -> (5,10) -> close
        let segments = vec![
            PathSegment::Line(Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 0.0 }),
            PathSegment::Line(Point { x: 10.0, y: 0.0 }, Point { x: 5.0, y: 10.0 }),
            PathSegment::Line(Point { x: 5.0, y: 10.0 }, Point { x: 0.0, y: 0.0 }),
            PathSegment::ClosePath,
        ];
        let area = signed_area_of_path(&segments);
        // Triangle area = base * height / 2 = 10 * 10 / 2 = 50
        assert!((area - 50.0).abs() < 1e-10);
        assert!(is_counter_clockwise(&segments));
    }

    #[test]
    fn test_signed_area_circle() {
        // Full circle with radius 5
        let radius = 5.0;
        let segments = vec![
            PathSegment::Arc(
                Point { x: 0.0, y: 0.0 },
                radius,
                0.0,
                360.0
            ),
        ];
        let area = signed_area_of_path(&segments);
        let expected = std::f64::consts::PI * radius * radius;
        assert!((area - expected).abs() < 1e-6, "Circle area should be π*r²");
    }

    #[test]
    fn test_signed_area_semicircle() {
        // Semicircle (arc from 0° to 180°) plus diameter
        let radius = 5.0;
        let segments = vec![
            PathSegment::Arc(
                Point { x: 0.0, y: 0.0 },
                radius,
                0.0,
                180.0
            ),
            PathSegment::Line(Point { x: -5.0, y: 0.0 }, Point { x: 5.0, y: 0.0 }),
            PathSegment::ClosePath,
        ];
        let area = signed_area_of_path(&segments);
        let expected = std::f64::consts::PI * radius * radius / 2.0;
        assert!((area - expected).abs() < 1e-6, "Semicircle area should be π*r²/2");
    }

    #[test]
    fn test_signed_area_mixed_path() {
        // Path with both lines and arcs
        // Simple path: line + semicircle
        let segments = vec![
            PathSegment::Line(Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 0.0 }),
            PathSegment::Arc(
                Point { x: 5.0, y: 0.0 },
                5.0,
                0.0,
                180.0
            ),
            PathSegment::Line(Point { x: 0.0, y: 0.0 }, Point { x: 10.0, y: 0.0 }),
            PathSegment::ClosePath,
        ];
        let area = signed_area_of_path(&segments);
        // Semicircle area = π*r²/2 = π*25/2 ≈ 39.27
        let expected = std::f64::consts::PI * 25.0 / 2.0;
        assert!((area.abs() - expected).abs() < 5.0, "Mixed path area should be approximately semicircle area");
    }
}

