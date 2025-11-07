#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, other: Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
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

#[derive(Copy, Clone, Debug)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl BoundingBox {
    pub fn from_points(points: &[Point]) -> Self {
        if points.is_empty() {
            return Self {
                min: Point { x: 0.0, y: 0.0 },
                max: Point { x: 0.0, y: 0.0 },
            };
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for p in points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        Self {
            min: Point { x: min_x, y: min_y },
            max: Point { x: max_x, y: max_y },
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y
    }
}
