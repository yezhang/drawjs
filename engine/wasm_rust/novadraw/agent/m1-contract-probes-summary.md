# M1 Contract Probes Summary

## Decision

- Milestone: M1 几何与 Graphics 基础
- Previous status: `in_progress`
- Recommended status: `contract_aligned`
- Decision time: 2026-06-11

M1 can move from `in_progress` to `contract_aligned`.

This decision is limited to the YAML contract and probes layer. It does not mean
`behavior_verified` or `complete`:

- `behavior_verified` still requires the product-layer checks defined by
  `doc/06-roadmap/product-deliverables.md`.
- `complete` still requires all applicable verification layers defined by
  `doc/06-roadmap/demo-matrix.md`.
- M1 has no independent demo, but it still needs product-layer existence and
  compatibility checks before `behavior_verified`.

## M1 Contract

Source of truth: `agent/draw2d-core-milestones.yaml`.

Scope:

- Point, Dimension, Rectangle, Insets, PointList, precision geometry, and Transform.
- Graphics draw, fill, text, image, clip, translate, scale, and state stack.
- Stroke, fill, alpha, line cap, line join, and color state.

Contracts:

- Geometry values must remain platform-independent.
- Graphics state must be push/pop scoped and nest safely.
- Clip and transform composition must be deterministic.

Probes:

- Geometry operation unit tests.
- Graphics state stack nesting tests.
- Clip and transform command snapshots.

## Probe Coverage

| Probe | Evidence | Result |
|-------|----------|--------|
| Geometry operation unit tests | `novadraw-geometry` has 42 passing tests covering `Vec2`, `Rectangle`, `Transform`, `Insets`, `PointList`, and `Precision` | Pass |
| Graphics state stack nesting tests | `graphics_state_stack_restores_nested_clip_transform_and_stroke_state` | Pass |
| Clip and transform command snapshots | `set_transform_and_reset_transform_emit_snapshot_commands`, `clip_reset_and_restore_are_visible_in_command_snapshot` | Pass |
| Text command snapshot | `global_alpha_is_scoped_and_applied_to_text_and_shapes`, `stroke_text_uses_stroke_style_and_current_font_snapshot` | Pass |
| Image command snapshot | `draw_image_records_destination_and_alpha_snapshot` | Pass |
| Alpha state semantics | `global_alpha_is_scoped_and_applied_to_text_and_shapes` | Pass |

## Delta Evidence

| Delta | Evidence | Result |
|-------|----------|--------|
| AD-020 Graphics state stack and clip-transform command snapshot | `NdCanvas` state stack, `SetTransform`, `ResetTransform`, `ResetClip`, Vello clip-depth interpreter, 3 render tests | Verified |
| AD-021 Seal mutable render command escape hatch | Removed mutable command Vec escape hatch, preserving command/state consistency | Verified |
| AD-022 Geometry missing types foundation | `Dimension`, `PointList`, `Precision`, `ApproxEq`, geometry tests | Verified |
| AD-023 Graphics text image alpha command support | `SetGlobalAlpha`, text/image command snapshots, alpha-scoped commands, 4 render tests | Verified |

## Verification

Commands:

```bash
cargo test -p novadraw-geometry
cargo test -p novadraw-render
```

Results:

- `cargo test -p novadraw-geometry`: 42/42 passed.
- `cargo test -p novadraw-render`: 7/7 passed.

## Residual Risks

- Text rendering is currently command-level and measurement-level only; real font
  shaping/rasterization belongs to later product/figure work.
- Image rendering is command-level and backend-independent; full resource
  management is intentionally out of M1 contract alignment.
- `measure_text` is deterministic but approximate; precision is sufficient for
  M1 contract probes, not for final text layout behavior.
- M1 should not be marked `behavior_verified` until product-layer checks are
  explicitly written and passed.

## Status Update

M1 is contract-aligned because all YAML probes have automated evidence and the
required substrate APIs are present at the command/value layer.

Next step:

- Create M1 product-layer existence checks before considering
  `behavior_verified`, or move to M2 if the workflow allows contract-aligned
  dependencies.
