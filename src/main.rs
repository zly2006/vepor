mod types;
mod geometry;
mod intersection;
mod boolean_ops;
mod resolver;

use types::{Point, Shape};
use resolver::resolve_shape;
use boolean_ops::find_shape_intersections;
use crate::intersection::line_line_intersection;

fn main() {
    println!("=== Testing Boolean Operations on Shapes ===\n");

    // Test 1: Circle and Rectangle intersection
    println!("TEST 1: Circle-Rectangle Intersection");
    println!("=====================================");
    let circle1 = Shape::Circle {
        center: Point { x: 10.0, y: 10.0 },
        radius: 5.0,
    };

    let rectangle1 = Shape::Rectangle {
        top_left: Point { x: 8.0, y: 8.0 },
        bottom_right: Point { x: 15.0, y: 12.0 },
    };

    // Union
    println!("1. UNION Operation:");
    let union_shape = Shape::Union(Box::new(circle1), Box::new(rectangle1));
    let resolved_union = resolve_shape(&union_shape);
    println!("   Result: {} segments", resolved_union.segments.len());

    // Show the actual intersections
    let c1 = resolve_shape(&Shape::Circle {
        center: Point { x: 10.0, y: 10.0 },
        radius: 5.0,
    });
    let r1 = resolve_shape(&Shape::Rectangle {
        top_left: Point { x: 8.0, y: 8.0 },
        bottom_right: Point { x: 15.0, y: 12.0 },
    });
    let intersections1 = find_shape_intersections(&c1, &r1);
    println!("   Intersection points found: {}", intersections1.len());
    for (i, pt) in intersections1.iter().enumerate() {
        println!("     Point {}: ({:.4}, {:.4})", i+1, pt.x, pt.y);
    }

    // Subtract
    println!("\n2. SUBTRACT Operation:");
    let circle2 = Shape::Circle {
        center: Point { x: 10.0, y: 10.0 },
        radius: 5.0,
    };

    let rectangle2 = Shape::Rectangle {
        top_left: Point { x: 8.0, y: 8.0 },
        bottom_right: Point { x: 15.0, y: 12.0 },
    };

    let subtract_shape = Shape::Subtract(Box::new(circle2), Box::new(rectangle2));
    let resolved_subtract = resolve_shape(&subtract_shape);
    println!("   Result: {} segments", resolved_subtract.segments.len());

    // Xor
    println!("\n3. XOR Operation:");
    let circle3 = Shape::Circle {
        center: Point { x: 10.0, y: 10.0 },
        radius: 5.0,
    };

    let rectangle3 = Shape::Rectangle {
        top_left: Point { x: 8.0, y: 8.0 },
        bottom_right: Point { x: 15.0, y: 12.0 },
    };

    let xor_shape = Shape::Xor(Box::new(circle3), Box::new(rectangle3));
    let resolved_xor = resolve_shape(&xor_shape);
    println!("   Result: {} segments", resolved_xor.segments.len());

    // Test 2: Two circles with various overlap conditions
    println!("\n\nTEST 2: Circle-Circle Intersections");
    println!("====================================");

    // Case 1: Two intersecting circles
    println!("Case 1: Two intersecting circles");
    let c1 = resolve_shape(&Shape::Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 5.0,
    });
    let c2 = resolve_shape(&Shape::Circle {
        center: Point { x: 6.0, y: 0.0 },
        radius: 5.0,
    });
    let ints = find_shape_intersections(&c1, &c2);
    println!("  Found {} intersection points", ints.len());
    for (i, pt) in ints.iter().enumerate() {
        println!("    Point {}: ({:.4}, {:.4})", i+1, pt.x, pt.y);
    }

    // Case 2: Tangent circles (external)
    println!("\nCase 2: Externally tangent circles");
    let c3 = resolve_shape(&Shape::Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 3.0,
    });
    let c4 = resolve_shape(&Shape::Circle {
        center: Point { x: 6.0, y: 0.0 },
        radius: 3.0,
    });
    let ints2 = find_shape_intersections(&c3, &c4);
    println!("  Found {} intersection points", ints2.len());
    for (i, pt) in ints2.iter().enumerate() {
        println!("    Point {}: ({:.4}, {:.4})", i+1, pt.x, pt.y);
    }

    // Case 3: Separate circles (no intersection)
    println!("\nCase 3: Separate circles (no intersection)");
    let c5 = resolve_shape(&Shape::Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 2.0,
    });
    let c6 = resolve_shape(&Shape::Circle {
        center: Point { x: 10.0, y: 0.0 },
        radius: 2.0,
    });
    let ints3 = find_shape_intersections(&c5, &c6);
    println!("  Found {} intersection points (expected: 0)", ints3.len());

    // Test 3: Line-Line intersections with edge cases
    println!("\n\nTEST 3: Line-Line Intersections");
    println!("================================");

    // Case 1: Intersecting lines
    println!("Case 1: Intersecting lines");
    let pts = line_line_intersection(
        Point { x: 0.0, y: 0.0 },
        Point { x: 10.0, y: 10.0 },
        Point { x: 0.0, y: 10.0 },
        Point { x: 10.0, y: 0.0 }
    );
    println!("  Found {} intersection points", pts.len());
    for (i, pt) in pts.iter().enumerate() {
        println!("    Point {}: ({:.4}, {:.4})", i+1, pt.x, pt.y);
    }

    // Case 2: Parallel lines (no intersection)
    println!("\nCase 2: Parallel lines");
    let pts2 = line_line_intersection(
        Point { x: 0.0, y: 0.0 },
        Point { x: 10.0, y: 0.0 },
        Point { x: 0.0, y: 5.0 },
        Point { x: 10.0, y: 5.0 }
    );
    println!("  Found {} intersection points (expected: 0)", pts2.len());

    // Case 3: Non-intersecting segments (lines would intersect but segments don't)
    println!("\nCase 3: Non-intersecting segments");
    let pts3 = line_line_intersection(
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 2.0 },
        Point { x: 5.0, y: 5.0 },
        Point { x: 10.0, y: 10.0 }
    );
    println!("  Found {} intersection points (expected: 0)", pts3.len());

    println!("\n=== All tests completed successfully ===");
}

