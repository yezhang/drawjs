# M1 Product Existence Checks

## Decision

- Milestone: M1 几何与 Graphics 基础
- Previous status: `contract_aligned`
- Recommended status: `behavior_verified`
- Decision time: 2026-06-11

M1 can move from `contract_aligned` to `behavior_verified`.

This decision covers the product-layer existence checks defined by
`doc/06-roadmap/product-deliverables.md`. M1 still has no independent demo in
`doc/06-roadmap/demo-matrix.md`, so this is not treated as a broad demo
completion claim.

## Product-Layer Coverage

| Product item | Evidence | Result |
|--------------|----------|--------|
| Geometry type names | `novadraw-geometry/tests/m1_product_existence.rs` imports `Point`, `PrecisionPoint`, `Rectangle`, `PrecisionRectangle`, `Dimension`, `PrecisionDimension`, `Insets`, `PointList`, `Vector`, `Transform`, and `AffineTransform` | Pass |
| Precision compatibility | `Precision*` aliases keep `ApproxEq` behavior through default `f64` geometry | Pass |
| Graphics state stack and styles | Existing render tests plus product tests cover `push_state`, `pop_state`, `restore_state`, color state, alpha, line width, and line style command snapshots | Pass |
| Graphics transforms and clipping | Existing render tests cover translate/scale/set/reset transform and clip/reset clip; product tests add `set_clip` coverage | Pass |
| Graphics draw/fill/text/image entries | Product tests cover rectangle, oval, polygon, text/string, image, and alpha command emission | Pass |

## Delta Evidence

| Area | Evidence | Result |
|------|----------|--------|
| Geometry aliases | `PrecisionPoint`, `PrecisionRectangle`, `PrecisionDimension`, `Vector`, and `AffineTransform` are exported from `novadraw-geometry` | Verified |
| Graphics compatibility methods | `NdCanvas` exposes product-layer snake_case entries such as `fill_rectangle`, `draw_rectangle`, `fill_oval`, `draw_oval`, `fill_polygon`, `draw_polygon`, `draw_text`, `draw_string`, `set_clip`, `set_alpha`, and style setters | Verified |
| Command snapshot | `LineStyle` is part of stroke command snapshots, allowing renderer-independent line style recording | Verified |

## Verification

Commands:

```bash
cargo fmt --check
cargo test -p novadraw-geometry
cargo test -p novadraw-render
cargo check --workspace
cargo clippy -p novadraw-geometry -p novadraw-render -- -D warnings
cargo check -p novadraw-render --features vello
```

Results:

- `cargo fmt --check`: passed.
- `cargo test -p novadraw-geometry`: 44/44 passed.
- `cargo test -p novadraw-render`: 9/9 passed.
- `cargo check --workspace`: passed.
- `cargo clippy -p novadraw-geometry -p novadraw-render -- -D warnings`: passed.
- `cargo check -p novadraw-render --features vello`: passed.

## Residual Risks

- `LineStyle` is currently recorded in backend-independent commands; Vello dash/dot
  rendering can be expanded later without changing the public product-layer entry.
- Real text shaping/rasterization and image resource management remain outside M1;
  M1 only requires command-layer and product-entry existence.
- M1 should not be marked `complete` until roadmap/demo matrix synchronization and
  documentation expectations are reconciled.

## Status Update

M1 is behavior-verified because both YAML contract probes and product-layer
existence checks now have repeatable automated evidence.

Next step:

- Start M2 Figure 树与盒模型, or first clean `BASELINE-002` so the full
  repository verification gate can become fully green.
