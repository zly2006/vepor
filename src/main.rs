#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

enum Shape {
    Circle { center: Point, radius: f64 },
    Rectangle { top_left: Point, bottom_right: Point },
    Union(Box<Shape>, Box<Shape>),
    Scale(Box<Shape>, f64),
    Subtract(Box<Shape>, Box<Shape>),
    Xor(Box<Shape>, Box<Shape>),
}

enum PathSegment {
    Line(Point, Point),
    Arc(Point, f64, f64, f64), // center, radius, start_angle, end_angle
    ClosePath, // closes the current path to the starting point using a straight line
}

struct ResolvedShape {
    segments: Vec<PathSegment>,
}

fn get_starting_point(segments: &Vec<PathSegment>) -> Option<Point> {
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
            PathSegment::ClosePath => continue,
        }
    }
    None
}

fn resolve_shape(shape: &Shape) -> ResolvedShape {
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
                    PathSegment::Arc(center, radius, start_angle, end_angle) => {
                        center.x = scale_center.x + (*factor) * (center.x - scale_center.x);
                        center.y = scale_center.y + (*factor) * (center.y - scale_center.y);
                        *radius *= *factor;
                    }
                    PathSegment::ClosePath => {}
                }
            }
            resolved
        }
        Shape::Union(shape1, shape2) => {
            // Placeholder implementation for union resolution
            let mut resolved1 = resolve_shape(shape1);
            let resolved2 = resolve_shape(shape2);
            resolved1.segments.extend(resolved2.segments);
            resolved1
        }
        Shape::Subtract(_, _) | Shape::Xor(_, _) => {
            // Placeholder for subtract and xor operations
            ResolvedShape { segments: vec![] }
        }
    }
}

fn main() {
    let x = 5;
}
