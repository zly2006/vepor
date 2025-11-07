## Gemini Coder Instructions

This document provides instructions for the Gemini Coder to interact with the `vepor` project.

### Project Overview

`vepor` is a Rust library for performing boolean operations (Union, Subtract, XOR) on 2D shapes with precise geometric intersection calculations. It is a command-line application that also provides a viewer to visualize the shapes and their intersections.

**Key Technologies:**
- Rust
- `eframe` and `egui` for the GUI viewer.

**Project Structure:**
The project is organized into several modules:
- `main.rs`: Main entry point with a demo for circle-arc intersection.
- `types.rs`: Core data types (`Point`, `Shape`, `PathSegment`, `ResolvedShape`).
- `geometry.rs`: Basic geometric utilities.
- `intersection.rs`: Intersection calculation functions.
- `boolean_ops.rs`: Boolean operations (Union, Subtract, XOR).
- `resolver.rs`: Shape resolution logic.
- `viewer.rs`: GUI viewer for visualizing shapes.

### Building and Running

**Build the project:**
```bash
cargo build
```

**Run the demo:**
The demo in `main.rs` calculates and visualizes the intersection of a circle and an arc.
```bash
cargo run
```

**Run tests:**
The project has a comprehensive test suite.
```bash
cargo test
```

### Development Conventions

- **Testing:** The project has a strong emphasis on testing. Each module has its own test suite. When adding new features, it is important to add corresponding tests.
- **Code Style:** The code follows standard Rust conventions. Use `cargo fmt` to format the code before committing.
- **Error Handling:** The project uses `Result` for error handling. Ensure that errors are handled properly.
- **Immutability:** The project favors immutability where possible. Use `&` and `&mut` to borrow and mutate data.
- **Modules:** The project is organized into modules with clear responsibilities. When adding new functionality, consider which module it belongs to or if a new module should be created.
