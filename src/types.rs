#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub enum Shape {
    Circle {
        center: Point,
        radius: f64,
    },
    Rectangle {
        top_left: Point,
        bottom_right: Point,
    },
    Union(Box<Shape>, Box<Shape>),
    Scale(Box<Shape>, f64),
    Subtract(Box<Shape>, Box<Shape>),
    Xor(Box<Shape>, Box<Shape>),
}

#[derive(Copy, Clone, Debug)]
pub enum PathSegment {
    Line(Point, Point),
    Arc(Point, f64, f64, f64), // center, radius, start_angle, end_angle
    ConnectedArc(Point, f64, f64, f64, Point, Point), // center, radius, start_angle, end_angle, start_point, end_point
    ClosePath,        // closes the current path to the starting point using a straight line
    DrawPoint(Point), // draws a single point (useful for marking intersection points)
}

pub struct ResolvedShape {
    pub segments: Vec<PathSegment>,
}
