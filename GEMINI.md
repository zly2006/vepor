## Gemini Coder Instructions

This document provides instructions for the Gemini Coder to interact with the `vepor` project.

### User Preferences and Key Requirements

*   **Drawing Modes:** The user wants to be able to draw shapes using both click-to-draw and drag-to-draw methods.
*   **Cancel with Escape:** The user wants to be able to cancel the current drawing operation by pressing the Escape key.
*   **Control Points:** The user wants to see control points for shapes, and these points should have a "snapping" effect for easier alignment.
*   **Scrollable UI:** The list of shapes in the UI should be horizontally scrollable to accommodate a large number of items.
*   **Preserve Functionality:** Do not remove existing functionality unless explicitly asked to do so.
*   **Use `replace` Tool:** For file modifications, use the `replace` tool instead of shell commands.
* 每次开始前都要执行检查git状态
* 不要尝试自己rollback。如果gradle成功及时commit，回滚自己的修改必须使用git来操作。
*   **Task Completion Notification:** Use `terminal-notifier -title "任务提醒" -message "任务已完成" -sound default` to notify the user upon task completion. The message text can be varied according to the work content.

### Current Progress

*   **Boolean Operations UI:** The UI and selection logic for all boolean operation tools (Intersection, Union, Difference, Xor) is implemented.
*   **Intersection Display:** When two shapes are selected with the "Intersection" tool, the intersection points are calculated and displayed.
*   **Boolean Operation Stubs:** The `compute_union`, `compute_subtract`, and `compute_xor` functions are placeholders. They are wired to the UI, but do not yet produce correct geometric results.

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
