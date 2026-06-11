# Architecture Inbox

用于收纳中途插入的突发任务，避免主线架构改进上下文丢失。

## 使用规则

- 每条突发任务单独记录
- 必须注明来源、影响范围、是否阻塞主线
- 如果任务与当前 delta 无关，不要直接插入主线 backlog

## Template

```text
- id: INBOX-YYYYMMDD-01
  source: user | bug | review | experiment
  summary: 一句话描述突发任务
  impact: low | medium | high
  blocks_current_delta: yes | no
  suggested_action: defer | split | switch
```

## Items

- id: INBOX-20260610-01
  source: user
  summary: Viewport 后续开发暂时搁置，保留 `clip_to_viewport` 自动截图失败现场，后续恢复时再排查
  impact: medium
  blocks_current_delta: no
  suggested_action: defer
  restart_point: 运行 `cargo run -p viewport-app -- --screenshot-clip`，检查 `apps/viewport-app/screenshot/*clip_to_viewport*.png`；从 `NdCanvas::clip_rect` 到 `VelloRenderer::push_clip_layer()` 的 clip 映射与 `perform_update -> repair -> render_to_iterative` 路径开始，不先改 `render_recursive.rs` / `render_iterative.rs` 主循环
