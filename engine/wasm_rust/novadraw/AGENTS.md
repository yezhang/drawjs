# Novadraw Project Guide for AI Agents

## Project Overview

**Novadraw** is a high-performance 2D graphics toolkit implemented in Rust with WebGPU support.
It is architecturally inspired by Eclipse Draw2D/GEF, aiming to be a general-purpose graphics framework.

### Key Characteristics

- **Language**: Rust (Edition 2024)
- **Graphics Backend**: Vello (WebGPU-based), with planned support for pluggable backends
- **Windowing/Events**: winit (for technical validation)
- **Text Rendering**: cosmic-text (planned)
- **Build Tool**: Cargo

## Workspace Structure

This is a Cargo workspace with the following crates:

```text
novadraw/                 # Main aggregation crate (public API facade)
├── novadraw-core/        # Core types: Color, basic primitives
├── novadraw-math/        # Math library: Vec3, Mat3
├── novadraw-geometry/    # 2D geometry: Vec2, Rect, Transform, Translatable
├── novadraw-render/      # Rendering abstraction: commands, context, backend traits
├── novadraw-scene/       # Scene graph, Figure interface, layout management
├── apps/editor/          # Example editor application
└── apps/vello-demo/      # Vello rendering demo
```

### Dependency Graph

```text
novadraw (facade)
├── novadraw-core
├── novadraw-math
├── novadraw-geometry
│   └── novadraw-math
├── novadraw-render (optional, vello feature)
│   ├── novadraw-core
│   ├── novadraw-math
│   └── novadraw-geometry
└── novadraw-scene (optional, vello feature)
    ├── novadraw-core
    ├── novadraw-math
    ├── novadraw-geometry
    └── novadraw-render

apps/editor               # Demo application
└── novadraw (with vello + debug_render features)
```

## Build Commands

### Development Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

### Run Example Editor

```bash
cargo run --package editor
```

### Run Vello Demo

```bash
cargo run --package vello-demo
```

### Code Quality Checks (Run Before Commit)

```bash
cargo fmt      # Format code
cargo check    # Compilation check
cargo clippy   # Linting
cargo test     # Run tests
```

### Generate Documentation

```bash
cargo doc --open
```

## Testing Strategy

- **Unit Tests**: Each crate has inline tests (marked with `#[cfg(test)]`)
- **Test Location**: Tests are co-located with source files in the same `.rs` files
- **Run All Tests**: `cargo test --workspace`
- **Coverage Areas**:
  - Core types: Color
  - Math: Vec3, Mat3
  - Geometry: Vec2, Rect, Transform, Translatable
  - Scene: SceneGraph, Viewport

## Code Style Guidelines

### Naming Conventions

- Types / Traits: `PascalCase`
- Functions / Variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Generic Parameters: Single uppercase letters (`T`, `U`)

### Documentation Standards

- Markdown headers (`#`, `##`, `###`, `####`) must have blank lines before and after
- No extra blank lines between list items
- Code blocks must use `rust` language annotation
- All `pub` types and functions must have doc comments (`///`)

### Visibility

- Public API requires documentation
- Internal implementation details should remain private
- Use `pub(crate)` for crate-internal sharing

### Error Handling

- Define custom error types using `thiserror` for critical errors
- Return `Option` for expected "empty states"
- **NEVER use `unwrap()` / `panic!` in production code**

## Critical Architecture Rules

### 1. Tree Traversal - NO RECURSION

**MANDATORY**: Tree traversals must use iterative stack-based implementation, never function recursion.

**Correct**:

```rust
let mut stack = vec![root_id];
while let Some(id) = stack.pop() {
    // Process node
    for &child in &block.children {
        stack.push(child);
    }
}
```

**Forbidden**:

```rust
fn visit(node: &Node) {
    // ...
    for child in &node.children {
        visit(child);  // NEVER DO THIS
    }
}
```

**Rationale**: Prevent stack overflow from deep hierarchies.

### 2. Child Propagation in SceneGraph

All child propagation operations must be implemented iteratively in `SceneGraph`, not in `RuntimeBlock` or `Figure`.

**Correct**:

```rust
impl SceneGraph {
    pub fn prim_translate(&mut self, block_id: BlockId, dx: f64, dy: f64) {
        let mut stack = vec![block_id];
        while let Some(id) = stack.pop() {
            // Update current node
            // ...
            // Push children to stack
            for &child_id in &block.children {
                stack.push(child_id);
            }
        }
    }
}
```

**Forbidden**:

```rust
impl Figure {
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.bounds.x += dx;
        self.bounds.y += dy;
        for child in &mut self.children {
            child.translate(dx, dy);  // NEVER DO THIS
        }
    }
}
```

### 3. Core Architecture Principles

1. **Figure is State-Less**: Figures only define rendering interface and geometry, no runtime state
2. **RuntimeBlock Manages State**: Visibility, selection state, hierarchy stored in RuntimeBlock
3. **Separation of Concerns**: Rendering and logic are separated for efficient massive graphics rendering
4. **Coordinate System**: Uses absolute coordinates by default, with local coordinate mode support

## Key Dependencies

| Crate     | Version   | Purpose                             |
|-----------|-----------|-------------------------------------|
| vello     | 0.7.0     | WebGPU rendering backend            |
| wgpu      | 26.0.1    | WebGPU bindings                     |
| winit     | 0.30.12   | Window creation and event handling  |
| glam      | 0.30.9    | Linear algebra types                |
| slotmap   | 1.0.7     | Efficient slot-based maps           |
| uuid      | 1.18.1    | Unique identifiers                  |
| serde     | 1.0.228   | Serialization                       |
| kurbo     | 0.11      | 2D curves and paths                 |

## Feature Flags

### novadraw

- `vello` (default): Enable Vello rendering backend
- `debug_render`: Enable debug rendering features

### novadraw-render

- `vello`: Enable Vello backend implementation

### novadraw-scene

- `vello`: Enable Vello integration
- `debug_render`: Enable debug rendering

## Project-Specific Conventions

### Forbiddens

- No temporary workarounds - fix root causes
- No unverified third-party crates
- No logging in rendering/hot paths
- No global state or Singletons
- No magic numbers in business code

### Performance Sensitive Paths

- Avoid memory allocation in render loops
- Use `SlotMap` / `Foa` for efficient data structures
- Consider `unsafe` optimizations for hot code (with comments)

## Design Decision Principles

1. **Progressive Design**: When making decisions, evaluate if the choice can be easily added incrementally later
   - High cost to add later or hard to extend: Design upfront
   - Can be smoothly added incrementally: Defer to later iteration

2. **Interface First**: For uncertain implementation details, define interface contracts first,
   delay concrete implementation

## Development Workflow

### Change Types

- **Major architectural changes**: Propose plan first, confirm before implementing
- **Bug fixes**: Understand root cause, explain before fixing
- **Code changes**: >50 lines should be split into multiple commits, each focused on a single change
- **New features**: Define interface first, then implement incrementally
- **Performance optimization**: Provide benchmark data to support claims

## Reference Materials

### Architecture Documentation (in `doc/`)

- `figure_core_concepts.md` - Eclipse Draw2D Figure core concepts
- `displaylist_design.md` - DisplayList intermediate layer design
- `coordinates.md` - Coordinate system documentation
- `figure_bounds.md` - Bounds system documentation
- `clip_principle.md` - Clipping principles
- `trampoline_rendering.md` - Trampoline rendering pattern

### External References

- draw2d/gef original implementation: `/Users/bytedance/Documents/code/GitHub/gef-classic`
- vello rendering library: `/Users/bytedance/Documents/code/GitHub/vello`

## Tooling Configuration

- **Rust Edition**: 2024
- **Markdown Lint**: Configured in `.markdownlint.yaml`
  - Line length limit: 120 characters
- **Serena Configuration**: `.serena/project.yml` (TypeScript focused, but project is Rust)

## Version Information

- Workspace Version: 0.1.0
- Rust Toolchain: 1.89.0
- Cargo: 1.89.0

## Security Considerations

- No unsafe code blocks without explicit safety comments
- Validate all external inputs (file paths, user data)
- Keep dependencies up to date (check with `cargo audit` when available)
