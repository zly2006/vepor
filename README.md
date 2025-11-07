# Vepor - 2D Shape Boolean Operations

A Rust library for performing boolean operations (Union, Subtract, XOR) on 2D shapes with precise geometric intersection calculations.

## Project Structure

```
src/
├── main.rs          # Main entry point with demo tests
├── types.rs         # Core data types (Point, Shape, PathSegment, ResolvedShape)
├── geometry.rs      # Basic geometric utilities
├── intersection.rs  # Intersection calculation functions
├── boolean_ops.rs   # Boolean operations (Union, Subtract, XOR)
└── resolver.rs      # Shape resolution logic
```

## Modules

### `types.rs`
Defines the core data structures:
- `Point`: 2D point with x, y coordinates
- `Shape`: Enum for different shape types (Circle, Rectangle, Union, Subtract, XOR, Scale)
- `PathSegment`: Enum for path segments (Line, Arc, ConnectedArc, ClosePath)
- `ResolvedShape`: Resolved shape consisting of path segments

### `geometry.rs`
Basic geometric utility functions:
- `distance()`: Calculate distance between two points
- `point_inside_circle()`: Check if a point is inside a circle
- `point_inside_rectangle()`: Check if a point is inside a rectangle
- `is_angle_in_arc()`: Check if an angle is within an arc range
- `get_starting_point()`: Get the starting point of a path
- `get_segment_midpoint()`: Get the midpoint of a segment

**Tests**: 5 unit tests

### `intersection.rs`
Intersection calculation functions with precise edge case handling:
- `line_line_intersection()`: Find intersections between two line segments
- `line_arc_intersection()`: Find intersections between a line segment and an arc
- `arc_arc_intersection()`: Find intersections between two arcs

**Tests**: 7 unit tests

### `boolean_ops.rs`
Boolean operations on shapes:
- `find_shape_intersections()`: Find all intersection points between two shapes
- `point_inside_shape()`: Check if a point is inside a shape using ray casting
- `compute_union()`: Compute the union of two shapes
- `compute_subtract()`: Subtract one shape from another
- `compute_xor()`: Compute the XOR of two shapes

**Tests**: 3 unit tests

### `resolver.rs`
Shape resolution logic:
- `resolve_shape()`: Resolve a shape into path segments

**Tests**: 4 unit tests

## Running Tests

Run all unit tests:
```bash
cargo test
```

Run specific module tests:
```bash
cargo test geometry::tests
cargo test intersection::tests
cargo test boolean_ops::tests
cargo test resolver::tests
```

Run the demo:
```bash
cargo run
```

## Test Coverage

Total: **19 unit tests**
- geometry: 5 tests
- intersection: 7 tests
- boolean_ops: 3 tests
- resolver: 4 tests

All tests verify:
- Basic geometric calculations
- Edge cases (parallel lines, tangent circles, etc.)
- Boundary conditions
- Angle range handling
- Shape containment logic

## Key Features

✅ **Precise intersection calculation**: All intersections computed using exact geometric equations  
✅ **Angle-aware arc handling**: Properly handles arc angle ranges, including arcs crossing 0°  
✅ **Edge case coverage**: Handles degenerate cases, tangent conditions, and boundary conditions  
✅ **Comprehensive testing**: 19 unit tests covering all critical paths  
✅ **Modular design**: Clean separation of concerns across modules  

## Example Usage

```rust
use vepor::types::{Point, Shape};
use vepor::resolver::resolve_shape;

let circle = Shape::Circle {
    center: Point { x: 10.0, y: 10.0 },
    radius: 5.0,
};

let rectangle = Shape::Rectangle {
    top_left: Point { x: 8.0, y: 8.0 },
    bottom_right: Point { x: 15.0, y: 12.0 },
};

// Union of two shapes
let union = Shape::Union(Box::new(circle), Box::new(rectangle));
let resolved = resolve_shape(&union);
```

## License

This project is for educational purposes.

