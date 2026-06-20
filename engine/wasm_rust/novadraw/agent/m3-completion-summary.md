# M3 Completion Summary

## Decision

- Milestone: M3 绘制遍历与裁剪闭环
- Previous status: `behavior_verified`
- Recommended status: `complete`
- Decision time: 2026-06-17

M3 can move from `behavior_verified` to `complete`.

This decision is limited to the recursive render mainline. Iterative render is a
deferred performance POC and remains archived at tag
`archive/render-iterative-poc-20260617`.

## API Semantics Closed

| API semantic | Evidence | Result |
|--------------|----------|--------|
| `paint.protocol` | Recursive render follows paintFigure -> paintClientArea/children -> paintBorder ordering | Pass |
| `graphics.context` | Render traversal uses push/restore/pop state around paint and clip phases | Pass |
| `clipping.strategy` | `ChildClippingStrategy` provides Draw2D-equivalent child clipping override across the active core Figure surface; deferred Figures remain classified for later M8/M10 gates | Pass |
| `border.protocol` | Border API, insets, clientArea impact, and paintBorder are available across the active core Figure surface; reusable builtin Figure parity belongs to M10 | Pass |
| `hit_test.search` | Paint-visible clientArea and hit-test descent share the same border-inset rule | Pass |

## Delta Evidence

| Delta | Evidence | Result |
|-------|----------|--------|
| AD-034 M3 border insets client-area clipping | Existing concrete Figure types expose border API; border insets feed clientArea; render clips children to inset clientArea | Verified |
| AD-035 M3 paint versus hit-test consistency tests | Paint clip, hit-test, and mouse event target descent share clientArea semantics | Verified |
| AD-036 M3 clip-app visual verification | Nested clip screenshot verified through visual workflow | Verified |
| AD-037 M3 iterative render POC archive | Iterative render removed from mainline and recovery tag exists | Verified |
| AD-038 M1-M3 completion API semantics | `ChildClippingStrategy` and border API are exposed on the active core Figure surface; additional existing Figures are explicitly classified as M8/M10 deferred surface | Verified |

## Verification

Commands:

```bash
cargo fmt --check
cargo test -p novadraw-scene
cargo clippy -p novadraw-scene -- -D warnings
```

Results:

- `cargo fmt --check`: passed.
- `cargo test -p novadraw-scene`: 164/164 + 5 integration + 3 doctests passed.
- `cargo clippy -p novadraw-scene -- -D warnings`: passed.

## Residual Risks

- Full damage repaint optimization belongs to M5/M8 and is not part of M3
  completion.
- Additional custom clipping strategies can be added later, but the M3 API
  extension point and default child-bounds behavior are now present on the
  active core Figure surface.
- Deferred viewport and reusable builtin Figures must implement the same border
  and clipping strategy surface before being counted as M8/M10 progress.
