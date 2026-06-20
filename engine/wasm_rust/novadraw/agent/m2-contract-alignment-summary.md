# M2 Contract Alignment Summary

## Decision

- Milestone: M2 Figure 树与盒模型
- Previous status: `in_progress`
- Recommended status: `complete`
- Decision time: 2026-06-12

M2 can move from `behavior_verified` to `complete`.

This decision is limited to M2's declared Draw2D API semantics. Full layout
algorithm set, full event state machine, and reserved extension semantics remain
outside M2 scope.

## M2 Contract

Source of truth: `agent/draw2d-core-milestones.yaml`.

Scope:

- Figure, FigureBlock, and FigureGraph roles.
- Parent and children topology.
- Bounds, insets, clientArea, visible, enabled, and z-order.
- Tree add, remove, and reparent invariants.
- addNotify/removeNotify equivalent lifecycle hooks.

Contracts:

- Draw2D A1 lightweight Figure tree axiom.
- Draw2D A2 tree as runtime backbone axiom.
- Draw2D A3 bounds unified geometry axiom.
- Draw2D A5 clientArea box model axiom.

Probes:

- Add, remove, and reparent no-cycle tests.
- Child order and z-order tests.
- Bounds, insets, and clientArea consistency tests.
- Visible and enabled propagation tests.

## Probe Coverage

| Probe | Evidence | Result |
|-------|----------|--------|
| Add, remove, and reparent no-cycle tests | `test_apply_pending_add_child_with_invalid_parent_has_no_side_effect`, `test_direct_add_child_to_invalid_parent_has_no_side_effect`, `test_try_add_child_to_invalid_parent_returns_none_without_side_effect`, `test_apply_pending_reparent_to_invalid_parent_keeps_original_tree`, `test_apply_pending_reparent_to_self_keeps_original_tree`, `test_apply_pending_reparent_to_descendant_keeps_original_tree`, `test_apply_pending_remove_child_with_wrong_parent_has_no_side_effects`, `test_apply_pending_reparent_with_duplicate_new_parent_entry_has_no_side_effects` | Pass |
| Child order and z-order tests | `test_child_order_appends_children_back_to_front`, `test_z_order_reorder_changes_topmost_hit_test_target`, `test_z_order_reorder_rejects_invalid_inputs_without_side_effects`, `test_hit_test_prefers_topmost_deepest_child` | Pass |
| Bounds, insets, and clientArea consistency tests | `test_client_area_resets_origin_for_coordinate_root`, `test_render_clips_non_local_coordinate_figure_to_client_area`, `test_hit_test_descends_only_through_parent_client_area`, `test_mouse_event_target_descends_only_through_parent_client_area`, parent-chain insets coordinate tests | Pass |
| Visible and enabled propagation tests | `test_effective_visibility_follows_parent_chain`, `test_effective_enabled_follows_parent_chain`, `test_repaint_skips_effectively_invisible_child`, hidden/disabled ancestor validation queue drain tests | Pass |

## Delta Evidence

| Delta | Evidence | Result |
|-------|----------|--------|
| AD-026 M2 topology add entry guard | Public add entry points reject invalid parent without allocation, UUID mutation, or update queue side effects | Verified |
| AD-027 M2 effective visible/enabled propagation | Local and effective state queries exist; repaint and validation honor parent-chain visible/enabled state | Verified |
| AD-028 M2 child order and z-order contract audit | Sibling order query and safe z-order reordering APIs exist; topmost hit-test follows draw2d reverse child search semantics | Verified |
| AD-029 M2 bounds/insets/clientArea consistency audit | child search for hit-test and mouse target descends only through parent clientArea | Verified |
| AD-030 M2 remove and reparent lifecycle contract audit | remove/reparent invalid paths avoid partial writes and preserve topology/update/interaction state | Verified |
| AD-038 M1-M3 completion API semantics | `Figure::on_attached/on_detached` exposes addNotify/removeNotify equivalents and `FigureGraph` calls them on add/remove/reparent | Verified |

## Verification

Commands:

```bash
cargo test -p novadraw-scene
cargo clippy -p novadraw-scene -- -D warnings
ruby agent/workflow-doctor.rb
bash agent/workflow-verify.sh --fast
```

Results:

- `cargo test -p novadraw-scene`: 163/163 + 3 integration + 3 doctests passed.
- `cargo clippy -p novadraw-scene -- -D warnings`: passed.
- `ruby agent/workflow-doctor.rb`: passed.
- `bash agent/workflow-verify.sh --fast`: passed.

## Residual Risks

- Direct public remove/reparent APIs are intentionally not introduced in M2;
  structural runtime mutation is currently exercised through pending mutations.
- Full layout algorithm set and full event state machine are explicitly outside
  M2 scope and belong to later milestones.
- The next milestone should not assume rendering traversal or clipping is
  complete; that belongs to M3.

## Status Update

M2 is complete within its scoped API semantics because all YAML probes have
automated evidence and the Figure tree now acts as the runtime backbone for
topology, z-order, visibility/enabled state, lifecycle hooks, coordinates,
validation entry points, and hit testing.

Next step:

- Continue M3 completion with real clipping strategy API semantics.
