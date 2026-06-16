# M2 Product Existence Checks

## Decision

- Milestone: M2 Figure 树与盒模型
- Previous status: `contract_aligned`
- Recommended status: `behavior_verified`
- Decision time: 2026-06-12

M2 can move from `contract_aligned` to `behavior_verified`.

This decision covers the product-layer existence checks defined by
`doc/06-roadmap/product-deliverables.md`. M2 still requires the `shapes-demo`
end-to-end/demo verification layer before it can move to `complete`.

## Product-Layer Coverage

| Product item | Evidence | Result |
|--------------|----------|--------|
| Five basic figures | `novadraw-scene/tests/m2_product_existence.rs` imports and constructs `RectangleFigure`, `EllipseFigure`, `PolygonFigure`, `RoundedRectangleFigure`, and `TriangleFigure` as `Box<dyn Figure>` | Pass |
| Figure/FigureBlock/FigureGraph roles | The product test uses public `FigureGraph` tree APIs, block queries, child order, z-index, visible/enabled state, and hit-test behavior from outside the crate | Pass |
| Three-phase paint protocol | The product test renders a parent/child marker scene and observes `paint_figure -> child paint -> child border -> parent border` command order | Pass |

## Delta Evidence

| Area | Evidence | Result |
|------|----------|--------|
| Product figure constructors | All five M2 product figures are constructible from crate-external integration tests | Verified |
| Runtime backbone API | `FigureGraph` exposes product-facing tree, box, z-order, hit-test, and effective state queries without direct storage mutation | Verified |
| Render protocol observability | `FigureGraph::render()` exposes the three-phase paint ordering through backend-independent `RenderCommand` snapshots | Verified |

## Verification

Commands:

```bash
cargo fmt --check
cargo test -p novadraw-scene
cargo clippy -p novadraw-scene -- -D warnings
ruby agent/workflow-doctor.rb
bash agent/workflow-verify.sh --fast
```

Results:

- `cargo fmt --check`: passed.
- `cargo test -p novadraw-scene`: 161/161 unit tests, 3/3 `m2_product_existence` tests, and 3/3 doctests passed.
- `cargo clippy -p novadraw-scene -- -D warnings`: passed.
- `ruby agent/workflow-doctor.rb`: passed.
- `bash agent/workflow-verify.sh --fast`: passed.

## Residual Risks

- M2 is not `complete`; `apps/shapes-demo` screenshot/demo verification remains
  the end-to-end layer in `doc/06-roadmap/demo-matrix.md`.
- Direct public remove/reparent APIs are intentionally not introduced in M2;
  structural runtime mutation remains covered through pending mutation paths.
- Full layout algorithms, full event state machine, and rendering traversal
  equivalence remain outside M2 and belong to later milestones.

## Status Update

M2 is behavior-verified because both YAML contract probes and product-layer
existence checks now have repeatable automated evidence.

Next step:

- Start M3 绘制遍历与裁剪闭环 contract work, while leaving the M2 demo layer
  for the later `complete` transition.
